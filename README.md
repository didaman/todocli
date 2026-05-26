# todocli

一个用 Rust 编写的命令行 Todo 任务管理工具，支持新增、查看、完成和删除任务，数据以 JSON 文件持久化存储。

## 功能特性

- **新增任务**：将一条待办事项写入本地存储
- **查看任务**：列出所有任务及其完成状态
- **标记完成**：将指定 ID 的任务标记为已完成
- **删除任务**：从列表中永久移除指定 ID 的任务

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
# 输出: Task { id: 1, task_name: "买牛奶", is_done: false }
#       Task { id: 2, task_name: "写周报", is_done: false }

# 将 id=1 的任务标记为已完成
todo done 1

# 删除 id=2 的任务
todo remove 2
```

## 项目结构

```
todocli/
├── Cargo.toml        # 项目配置与依赖声明
├── task.json         # 任务数据持久化文件（运行后自动生成）
└── src/
    ├── main.rs       # 程序入口，解析参数并调用 run()
    ├── lib.rs        # 公共接口：Config 构建与命令分发
    └── task.rs       # 任务数据层：JSON 读写与业务逻辑
```

### 模块职责

- **`main.rs`**：负责收集命令行参数，构建 `Config`，调用 `todocli::run()`，统一处理错误并以非零退出码退出。
- **`lib.rs`**：定义 `Command` 枚举（`Add` / `List` / `Done` / `Remove`）和 `Config` 结构体，通过 `Config::build()` 解析参数，`run()` 将命令分发到 `task` 模块。
- **`task.rs`**：持有 `Task` 结构体，实现 `load_or_init_tasks()`（读取或初始化 `task.json`）和 `save_task()`（序列化写回），对外暴露四个操作函数。

## 数据格式

任务以 JSON 数组的形式存储在项目根目录下的 `task.json` 文件中：

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

## 依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| [serde](https://crates.io/crates/serde) | 1.0 | 序列化 / 反序列化框架，启用 `derive` feature |
| [serde_json](https://crates.io/crates/serde_json) | 1.0 | JSON 格式支持 |

## 已知限制与改进方向

- `task.json` 路径硬编码为 `./task.json`（当前工作目录），不同目录运行会产生不同数据文件
- ID 自增逻辑依赖末尾元素，删除任务后 ID 不会复用，长期使用可能出现较大间隔
- 缺少任务编辑（`edit`）功能
- 缺少单元测试与集成测试覆盖

## License

MIT
