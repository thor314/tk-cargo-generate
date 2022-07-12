let project_name = variable::get("project-name").to_upper();
print("project name is" + project_name);
debug(`project name: ${project_name}`);

// create a subdir called `project-name`

// move {tests, src, benches, README.md, and maybe .env}, into `project-name`
file::rename("tests", `${project_name}/tests`);
file::rename("src", `${project_name}/src`);
file::rename("README.md", `${project_name}/README.md`);
if file::exists(".env") { file::rename(".env", `${project_name}/.env`); }

// move workspace/{README.md,Cargo.toml} to top level, and delete workspace dir
file::rename("workspace/Cargo.toml", "Cargo.toml");
file::rename("workspace/README.md",  "README.md");
file::delete("workspace");
