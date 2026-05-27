# todocli

一个用 Rust 编写的命令行 Todo 任务管理工具，支持新增、查看、完成和删除任务。

默认情况下，任务会持久化到 SQLite 数据库中。项目也保留了 JSON 存储实现，方便学习、对照或临时切换。

## 功能特性

- **新增任务**：将一条待办事项写入本地存储
- **查看任务**：列出所有任务及其完成状态
- **标记完成**：将指定 ID 的任务标记为已完成
- **删除任务**：从列表中永久移除指定 ID 的任务
- **SQLite 持久化**：默认使用本地 SQLite 数据库保存任务
- **JSON 后端兼容**：通过环境变量切回 JSON 文件存储

## 快速开始

### 前置条件

- [Rust](https://www.rust-lang.org/tools/install) 工具链（推荐通过 `rustup` 安装）

### 构建与安装

```bash
git clone <仓库地址>
cd todocli
cargo build --release
```

编译产物位于 `target/release/todo`，可将其复制到 `$PATH` 中的某个目录以全局使用：

```bash
cp target/release/todo ~/.local/bin/
```

### 直接运行（开发调试）

```bash
cargo run -- <子命令> [参数]
```

如果想使用旧的 JSON 存储后端进行对照，可以设置 `TODOCLI_STORAGE=json`：

```bash
TODOCLI_STORAGE=json cargo run -- list
```

## 使用方法

```
todo <子命令> [参数]
```

| 子命令 | 参数 | 说明 | 示例 |
|--------|------|------|------|
| `list` | 无 | 列出所有任务 | `todo list` |
| `add` | `<任务名称>` | 新增一条任务 | `todo add "buy milk"` |
| `done` | `<id>` | 将指定任务标记为已完成 | `todo done 1` |
| `remove` | `<id>` | 删除指定任务 | `todo remove 2` |

### 示例

```bash
# 添加任务
todo add "买牛奶"
todo add "写周报"

# 查看任务列表
todo list
# 输出:
# 1. 买牛奶
# 2. 写周报

# 将 id=1 的任务标记为已完成
todo done 1

# 删除 id=2 的任务
todo remove 2
```

## 项目结构

```
todocli/
├── Cargo.toml        # 项目配置与依赖声明
└── src/
    ├── main.rs       # 程序入口，解析参数并调用 run()
    ├── lib.rs        # 公共接口：Config 构建与命令分发
    ├── task.rs       # Task 数据结构与显示格式
    └── repository/
        ├── mod.rs    # TaskRepository trait 与后端导出
        ├── sqlite.rs # SQLite 存储实现，默认后端
        └── json.rs   # JSON 存储实现，兼容后端
```

### 模块职责

- **`main.rs`**：负责收集命令行参数，构建 `Config`，调用 `todocli::run()`，统一处理错误并以非零退出码退出。
- **`lib.rs`**：定义 `Command` 枚举（`Add` / `List` / `Done` / `Remove`）和 `Config` 结构体，通过 `Config::build()` 解析参数，`run()` 选择存储后端并分发命令。
- **`task.rs`**：定义 `Task` 结构体，并实现命令行列表输出使用的 `Display`。
- **`repository/mod.rs`**：定义 `TaskRepository` trait，统一 `list`、`add`、`remove`、`done` 四个操作。
- **`repository/sqlite.rs`**：默认存储后端。负责创建 SQLite 数据库文件、建表，并执行增删改查。
- **`repository/json.rs`**：JSON 存储后端。设置 `TODOCLI_STORAGE=json` 时使用。

## 存储后端

### SQLite（默认）

默认后端是 `SqliteTaskRepository`。数据库文件名为 `tasks.sqlite3`，位置由 [`directories`](https://crates.io/crates/directories) 的 `ProjectDirs` 决定，也就是当前操作系统推荐的应用数据目录。

第一次执行命令时，程序会自动创建数据库文件和下面这张表：

```sql
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_name TEXT NOT NULL,
    is_done INTEGER NOT NULL DEFAULT 0 CHECK (is_done IN (0, 1))
);
```

| 字段 | SQLite 类型 | Rust 类型 | 说明 |
|------|-------------|-----------|------|
| `id` | `INTEGER PRIMARY KEY AUTOINCREMENT` | `u32` | 由 SQLite 自动生成的新任务 ID |
| `task_name` | `TEXT NOT NULL` | `String` | 任务描述文本 |
| `is_done` | `INTEGER NOT NULL` | `bool` | `0` 表示待完成，`1` 表示已完成 |

### JSON（兼容）

设置 `TODOCLI_STORAGE=json` 后，程序会使用 `JsonTaskRepository`。JSON 文件名为 `task.json`，同样放在系统推荐的应用数据目录中。

```json
[
  { "id": 1, "task_name": "买牛奶", "is_done": false },
  { "id": 2, "task_name": "写周报", "is_done": true }
]
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `u32` | 自增整数，新任务 ID = 末尾任务 ID + 1 |
| `task_name` | `String` | 任务描述文本 |
| `is_done` | `bool` | `false` 待完成 / `true` 已完成 |

## 测试

```bash
cargo test
```

当前测试覆盖了 SQLite 后端的核心流程：新增任务、标记完成、删除任务、再次读取任务列表。

## 依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| [directories](https://crates.io/crates/directories) | 5 | 获取跨平台应用数据目录 |
| [rusqlite](https://crates.io/crates/rusqlite) | 0.37 | SQLite 数据库访问，启用 `bundled` feature 以避免依赖系统 SQLite |
| [serde](https://crates.io/crates/serde) | 1.0 | 序列化 / 反序列化框架，启用 `derive` feature |
| [serde_json](https://crates.io/crates/serde_json) | 1.0 | JSON 格式支持 |

## 已知限制与改进方向

- 还没有从旧 JSON 数据自动迁移到 SQLite 的流程
- SQLite 后端的 ID 由数据库自增生成，删除任务后 ID 不会复用
- 缺少任务编辑（`edit`）功能
- 缺少命令行层面的集成测试覆盖

## License

MIT
