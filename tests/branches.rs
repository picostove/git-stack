mod fixture;

use git_stack::git::*;

fn no_protect() -> git_stack::git::ProtectedBranches {
    git_stack::git::ProtectedBranches::new(vec![]).unwrap()
}

fn protect() -> git_stack::git::ProtectedBranches {
    git_stack::git::ProtectedBranches::new(vec!["master"]).unwrap()
}

mod test_branches {
    use super::*;

    #[test]
    fn test_all() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let branches = Branches::new(repo.local_branches());
        let result = branches.all();
        let mut names: Vec<_> = result
            .iter()
            .flat_map(|(_, b)| b.iter().map(|b| b.name.as_str()))
            .collect();
        names.sort_unstable();

        assert_eq!(
            names,
            [
                "base",
                "feature1",
                "feature2",
                "initial",
                "master",
                "off_master"
            ]
        );
    }

    #[test]
    fn test_descendants() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let base_oid = repo.resolve("base").unwrap().id;

        let branches = Branches::new(repo.local_branches());
        let result = branches.descendants(&repo, base_oid);
        let mut names: Vec<_> = result
            .iter()
            .flat_map(|(_, b)| b.iter().map(|b| b.name.as_str()))
            .collect();
        names.sort_unstable();

        // Should pick up master (branches off base)
        assert_eq!(
            names,
            ["base", "feature1", "feature2", "master", "off_master"]
        );
    }

    #[test]
    fn test_dependents() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let base_oid = repo.resolve("base").unwrap().id;
        let head_oid = repo.resolve("feature1").unwrap().id;

        let branches = Branches::new(repo.local_branches());
        let result = branches.dependents(&repo, base_oid, head_oid);
        let mut names: Vec<_> = result
            .iter()
            .flat_map(|(_, b)| b.iter().map(|b| b.name.as_str()))
            .collect();
        names.sort_unstable();

        // Shouldn't pick up master (branches off base)
        assert_eq!(names, ["base", "feature1", "feature2"]);
    }

    #[test]
    fn test_branch() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let base_oid = repo.resolve("base").unwrap().id;
        let head_oid = repo.resolve("feature1").unwrap().id;

        let branches = Branches::new(repo.local_branches());
        let result = branches.branch(&repo, base_oid, head_oid);
        let mut names: Vec<_> = result
            .iter()
            .flat_map(|(_, b)| b.iter().map(|b| b.name.as_str()))
            .collect();
        names.sort_unstable();

        // Shouldn't pick up feature1 (dependent) or master (branches off base)
        assert_eq!(names, ["base", "feature1"]);
    }

    #[test]
    fn test_protected() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let protect = protect();
        let branches = Branches::new(repo.local_branches());
        let result = branches.protected(&protect);
        let mut names: Vec<_> = result
            .iter()
            .flat_map(|(_, b)| b.iter().map(|b| b.name.as_str()))
            .collect();
        names.sort_unstable();

        assert_eq!(names, ["master"]);
    }
}

mod test_find_protected_base {
    use super::*;

    #[test]
    fn test_no_protected() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let protect = no_protect();
        let branches = Branches::new(repo.local_branches());
        let protected = branches.protected(&protect);

        let head_oid = repo.resolve("base").unwrap().id;

        let branch = find_protected_base(&repo, &protected, head_oid);
        assert!(branch.is_none());
    }

    #[test]
    fn test_protected_branch() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let protect = protect();
        let branches = Branches::new(repo.local_branches());
        let protected = branches.protected(&protect);

        let head_oid = repo.resolve("off_master").unwrap().id;

        let branch = find_protected_base(&repo, &protected, head_oid);
        assert!(branch.is_some());
    }

    #[test]
    fn test_protected_base() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let protect = protect();
        let branches = Branches::new(repo.local_branches());
        let protected = branches.protected(&protect);

        let head_oid = repo.resolve("base").unwrap().id;

        let branch = find_protected_base(&repo, &protected, head_oid);
        assert!(branch.is_some());
    }
}
