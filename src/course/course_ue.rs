//! # course UE
//! 
//! 浅析 UE 的设计架构及理念，与最佳实践 GamePlay GameFeatures
//! 
//! see [InsideUE](https://www.zhihu.com/people/fjz13/posts)
//! 
//! 
//! ## GamePlay =========
//! 
//! 
//! ## 从零到一的基础 UObject & Actor & Component
//! 
//! ### UObject
//! 
//! 在 C++ 基础上提供增强：GC、元数据、反射、序列化、编辑器可见、类默认对象等
//! 
//! ### Actor 演员
//! 
//! 在 UObject 基础上增加：Replication（网络复制能力）、Spawn（游戏世界中的生命周期）、Tick（帧刷新）
//! 
//! 注意： Actor 并不意味着就是可见的（不带Transform），可以代表游戏世界的某种信息、状态、规则
//! 
//! ### Component 组件
//! 
//! 随着系统复杂， Actor 数量和功能越来越多， UE 参考隔壁 Unity ，把各种功能抽象为 Component （没错，Unity也是ECS架构）
//! - UActorComponent 功能的载体，一定程度的嵌套组装能力 (SceneComponent)
//! - AActor 基础的游戏对象， Component 的容器
//! 
//! 
//! ## 一到一百的世界构成 Level & World
//! 
//! 每个游戏引擎对游戏世界的构成有不同看法：
//! - Cocos2dx 会认为游戏世界是由 Scene 组成的， Scene 再由一个个 Layer 层叠表现，然后再有一个 Director 来导演整个游戏
//! - Unity 觉得世界也是由 Scene 组成的，然后一个 Application 来扮演上帝来 LoadLevel ，后来换成了 SceneManager
//! - UE 中把这种拆分叫做关卡 Level ，由一个或多个 Level 组成一个 World
//! 
//! ### Level 关卡
//! 
//! Level 继承于 UObject
//! 
//! 相关的：
//! - ALevelScriptActor 允许在关卡里编写脚本，控制各个 Actor
//! - AWorldSettings 记录了 Level 的各个关照、物理、GameMode等属性
//! 
//! ### World 世界
//! 
//! 各个 Level 拼接起来就是 World 世界，划分的逻辑有助于动态加载和开发时团队协作
//! 
//! World 由 一个 PersistentLevel 和 多个 subLevels 组成
//! 
//! 
//! ## 世界之上的顶层设计 WorldContext & GameInstance & Engine
//! 
//! ### WorldContext
//! 
//! 一个游戏也可能存在多个 World ，比如UE编辑器本身就也是一个World，里面显示的游戏场景也是一个World
//! 
//! UE 用来管理和跟踪这些 World 的工具就是 WorldContext：
//! - 编辑器点击播放，就是从Preview切换到PIE(Play In Editor)，这时FWorldContext就用来保存切换过程信息和目标World上下文信息
//! - 一般来说，对于独立运行的游戏，WorldContext只有唯一个
//! - 一般来说，不需要直接操作到这个类，引擎内部已经处理好各种World的协作
//! 
//! ### GameInstance
//! 
//! GameInstance里会保存着当前的WorldConext和其他整个游戏的信息
//! 
//! ### Engine
//! 
//! UEngine 分化出了两个子类： UGameEngine 和 UEditorEngine
//! 
//! 
//! ## Pawn
//! 
//! 兵卒、棋子，任何可与玩家交互的 Actor （如植物建筑等就不算）
//! 
//! - DefaultPawn 默认带了 DefaultPawnMovementComponent、spherical_CollisionComponent、StaticMeshComponent
//! - SpectatorPawn 因为 UE 做 FPS 起家，有观战需求，给予相机漫游的能力
//! - Character 人形角色，大部分的角色都是人形的，相当于一个常用的增强
//! 
//! 
//! ## Controller
//! 
//! 专注于 Pawn 的控制（Pawn仅具备可控制的概念），继承自 Actor
//! - 多个实例，可复制、互不干扰（多个可控制角色同时存在）
//! - 与 Pawn 多对多的关系（RTS或多人协作游戏）（这点UE原生不支持，但也能自己扩展）
//! - 可独立存在，运行时挂载释放 Pawn 的能力
//! - 事件响应、帧响应
//! - 可探查世界的对象
//! - 可网络同步
//! 
//! ### PlayerController
//! 
//! - Input系统
//! - Camera的管理
//! - HUD显示
//! - UPlayer关联（本地或网络）
//! 
//! ### AIController
//! 
//! - Navigation 用于AI根据导航寻路
//! - AI组件，运行启动行为树，使用黑板数据，探索周围环境
//! - Task系统，让AI去完成一些任务
//! 
//! 
//! ## GameMode & GameState
//! 
//! ### GameMode
//! 
//! 在思考何为 Level 关卡中，认为场景是表示、玩法是逻辑。在 UE 的思想中，World更多是逻辑的概念，而Level是资源场景表示。
//! 
//! 当谈论Level的业务逻辑控制的时候，实际上谈的是World的业务逻辑。这个 "WorldController" 在 UE3 中叫 GameInfo ，在 UE4 中改名为 GameMode
//! 
//! 其作用大致分为：
//! - Class 登记： PalyerPawn HUD PlayerController GameState PlayerState 等等
//! - Pawn 加载释放
//! - 游戏进度
//! - Level 切换（包括 CG ）
//! - 多人游戏中的协同（连接后一起开始）
//! 
//! P.S. 多个 Level 配置不同 GameMode 时，只会为第一次创建 World 时加载 PersistentLevel 的时候创建 GameMode
//! 
//! ### GameState
//! 
//! GameState 保存当前游戏的状态数据，网络环境里也可以 Replicated 到多个 Client 上面去
//! 
//! ### GameSession
//! 
//! 在网络联机游戏中针对Session使用的一个方便的管理类，并不存储数据
//! 
//! 
//! ## Player
//! 
//! 到此处，已经完成了游戏世界的搭建，剩下的就是玩家进入其中游玩，根据游戏类型有以下区别：（举例）
//! - 玩家数目是单人还是多人
//! - 网络环境是只本地还是联网
//! - 窗口显示模式是单屏还是分屏
//! - 输入模式是共用设备还是分开控制（比如各有手柄）
//! 
//! ### UPlayer
//! 
//! 继承于 UObject （Player是比World更高一级的对象，LevelWorld可以切换），与 APlayerController 关联
//! 
//! ### ULocalPlayer
//! 
//! 继承于 UPlayer ，多了 Viewport 相关（渲染），并实例化了 APlayerController
//! 
//! ### UNetConnection
//! 
//! 在UE里，一个网络连接也是个Player（此处不深入）
//! 
//! 
//! ## GameInstance
//! 
//! GamePlay 的管理者，在 GameEngine 里创建（可以有多个），其内部接口大致有四类
//! - 引擎的初始化加载，Init和ShutDown等
//! - Player的创建，如CreateLocalPlayer，GetLocalPlayers之类的
//! - GameMode的重载修改
//! - OnlineSession的管理
//! 
//! 一个 GameInstance 包含：
//! - Worlds
//! - Players （允许动态增删）
//! - UI 区别于World之外的系统，暂且先放在这里，以后可能会单独拆出来
//! - 全局配置、游戏之外的三方逻辑等
//! 
//! ### SaveGame
//! 
//! 序列化存储（本地网络）
//! 
//! 
//! ## 总结
//! 
//! ### 游戏架构
//! 
//! 1. Actor 通过 Component 组装功能
//! 1. 各种 Actor 子类又组装成了 Level
//! 1. 一个个的 Level ，又进一步组装成了 World
//! 1. World 之间的切换，用了一个 WorldContext 来保存切换的过程信息
//! 1. 在往上就是游戏唯一 GameInstance ，由 GameEngine 对象管理着
//! 1. 到此游戏加载完成，需要玩家加入， GameInstance 下保存着 Player
//! 1. Player 有 PlayerController 和控制着的 Pawn 和摄像头
//! 1. 最后在 Engine 的 Tick 心跳脉搏驱动下开始一帧帧的逻辑更新和渲染
//! 
//! ### Model数据、Controller逻辑、View表现
//! 
//! ```txt
//! View表现          Controller逻辑          Model数据
//! 
//! GameEngine
//! GameInstance --------------------------- SaveGame
//!              -- Player 
//! WorldContext     |
//! World -----------|------- GameMode ------------- GameState
//! Level -----------|------- LevelScript     |   |
//!       -----------|------------------------|- WorldSettings
//! Pawn ---------- Controller ------------- PlayerState
//! Actor ------------------- Component
//! ```
//! 
//! 
//! ## Subsystems
//! 
//! GamePlay架构里的新增功能，其实感觉也是参考 ECS 架构中的 System ，就是基于类的思想框架给自动实现了单例模式，而不用去继承GameInstance
//! 
//! 该框架允许从五个类中选择一个来定义子类，且具有不同生命周期（根据其依存的哪种对象）（框架会自动实例化和释放）
//! - UEditSubsystem
//! - UEnginSubsystem
//! - UGameInstanceSubsystem
//! - UWorldSubsystem
//! - ULocalPlayerSubsystem
//! 
//! 有助于理解的一致性，UE里如果提到复用的理解一致性：
//! - 通用功能的复用：从各个ActorComponent里查看，这一层代表的往往是跟“游戏逻辑”无关的可复用功能（位置移动等）。
//! - 业务逻辑的复用：从Subsystem来查找，其代表就是游戏逻辑相关的可复用部分。
//! 
//! 
//! ## GameFeatures =========
//! 
//! 
//! ## 发展由来
//! 
//! 现有的工具
//! - 引擎的内建 GamePlay 框架
//! - GameAbilitySystem 技能框架
//! - Subsystem 系统
//! 
//! GameFeatures 是一种特殊的插件，随着 UE5 一起来到
//! - Plugin 插件更多是基础功能上的（数据库、网络通信）
//! - GameFeatures 玩法上的插件（底层也是基于Plugin机制实现）
//! 
//! GameFeatures 解决的需求
//! - 一个项目运营到一定时期才会遇到的问题：赛季更新、内容迭代等，会使得代码变得难以维护（是的，也是为了解耦合，不过是游戏玩法上的）
//! - 具备独立性（开发测试成本低）
//! - 协作和快速迭代，以及热更新
//! 
//! 
//! ## 初始化
//! 
//! 核心概念
//! - GameFeature，缩写为GF，代表一个GameFeature插件。
//! - CoreGame，把游戏的本体Module和GameFeature相区分开，即还没有GF发挥作用的游戏本体。
//! - UGameFeatureData，缩写为GFD，游戏功能的纯数据配置资产，用来描述了GF要做的动作，与GF同名。
//! - UGameFeatureAction，缩写GFA，单个动作。引擎已经内建了几个Action，我们也可以自己扩展。
//! - UGameFeatureSubsystem，缩写为GFS，GF框架的管理类，全局的API都可以在这里找到。
//! - UGameFeaturePluginStateMachine，缩写为GFSM，每个GF插件都关联着一个状态机来管理自身的加载卸载逻辑。
//! - UGameFrameworkComponentManager，缩写为GFCM，支撑AddComponent_Action作用的管理类，记录了为哪些Actor身上添加了哪些Component，以便在GF卸载的时候移除掉。
//! 
//! 该框架的核心管理类是UGameFeaturesSubsystem，它是继承自EngineSubsystem，这意味着它是跟随着引擎启动的。
//! - Editor时，内部管理的GF状态是跟着引擎编辑器一起的，停止PIE播放，GF插件也依然是保持加载状态的。
//! - 实践中推崇手动在游戏中通过API来激活或反激活GF，而不是利用编辑器上的UI按钮。
//! 
//! 加载策略Policy
//! - 默认的策略对象UDefaultGameFeaturesProjectPolicies，会加载所有的GF插件
//! - 可自定义策略，如测试，赛季、版本
//! 
//! 
//! ## 状态机
//! 
//! 主要关注以下状态，依次渐进、可双向往返
//! - Installed
//!   - 检查存在性（这里也可web下载），加载 C++ 模块 dll 完成之后的目标状态
//! - Registered
//!   - 注册到GFS里去，加载配置ini文件和GFD资产，并触发其中定义Action的OnGameFeatureRegistering（静态的预先逻辑）
//! - Loaded
//!   - 加载插件的运行时的ini，并预先加载一些资产（不会全部加载，其他的资产是在激活阶段根据Action的执行按需加载的）
//! - Active
//!   - 触发Action的OnGameFeatureActivating（Deactivating是OnGameFeatureDeactivating）
//! 
//! 
//! ## 最频繁使用的GFA：UGameFeatureAction_AddComponents
//! 
//! 主要是给CoreGame里的Actor添加组件（注意不要产生冲突）。
//! 
//! ### 作用机制
//! 
//! 是在 ModularGameplay 模块中 GFCM(UGameFrameworkComponentManager) 来实现的（注意到它是从 GameInstanceSubsystem继 承下来的，因此它是只能在游戏内运行时使用的）。
//! 
//! GFCM 内部的实现还是蛮复杂和精巧的，可以做到在一个GF激活后，会把激活前已经存在场景中Actor，还有激活后新生成的Actor，都会被正确的添加上Component。
//! 
//! 注意：只有那些调用了AddReceiver的Actor才可以正常的被AddComponents
//! - todo <https://zhuanlan.zhihu.com/p/492893002>
//! 