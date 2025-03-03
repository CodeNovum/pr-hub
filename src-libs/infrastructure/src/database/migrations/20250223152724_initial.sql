CREATE TABLE git_repositories 
(
		id INTEGER PRIMARY KEY, 
		name TEXT NOT NULL, 
		context TEXT NOT NULL, 
		is_active BOOLEAN NOT NULL DEFAULT 0,
		git_provider TEXT NOT NULL CHECK(git_provider IN ('azuredevops')),
		pat_secret_key TEXT NOT NULL,
		UNIQUE(name, context, git_provider)
);
