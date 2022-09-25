## 创建项目
```bash
cargo new {project-name}
```
## 构建项目（在项目根目录下执行）
```bash
cargo build
```
## 运行项目（在项目根目录下执行）
```bash
cargo run
```
## vscode 调试
launch.json 文件。
```json
{
    "version": "0.2.0",
    "configurations": [
        
        {
            "name": "Debug",
            "type": "gdb",
            "request": "launch",
            "target": "./target/debug/${workspaceFolderBasename}",
            "cwd": "${workspaceRoot}",
            "valuesFormatting": "parseText",
            "preLaunchTask": "Build"
        }
    ]
}
```

task.json 文件。
```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Build",
            "type": "shell",
            "command": "cargo build"
        }
    ]
}
```