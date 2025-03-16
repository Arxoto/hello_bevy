//! # GameFeatures
//! 
//! GameFeatures 是 UE5 推出的一个支持动态转载游戏玩法的框架
//! - [【功能介绍】UE5中的模块化游戏功能：即插即用，虚幻之道（官方字幕）](https://www.bilibili.com/video/BV1dL4y1h7YW/)
//! - [【英文直播】模块化游戏功能（官方字幕）](https://www.bilibili.com/video/BV1g34112799/)
//! - [【UOD2021】虚怀若谷-模块化游戏功能框架 Epic Games 大钊（官方字幕）](https://www.bilibili.com/video/BV1j34y1B7Nf/)
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
//! ## 最频繁使用的GFA： UGameFeatureAction_AddComponents
//! 
//! 主要是给CoreGame里的Actor添加组件（注意不要产生冲突）。
//! 
//! ### 作用机制
//! 
//! 是在 ModularGameplay 模块中 GFCM(UGameFrameworkComponentManager) 来实现的（注意到它是从 GameInstanceSubsystem继 承下来的，因此它是只能在游戏内运行时使用的）。
//! 
//! GFCM 内部的实现还是蛮复杂和精巧的，可以做到在一个GF激活后，会把激活前已经存在场景中Actor，还有激活后新生成的Actor，都会被正确的添加上Component。
//! 
//! 注意：只有那些调用了 AddReceiver 的 Actor 才可以正常的被 AddComponents
//! 
//! 对于新增的 Actor 如何添加 Component
//! - GFCM 里的成员变量 ReceiverClassToComponentClassMap 记录了 ActorClass-ComponentClass 集合的映射（某一类型的 Actor 应该添加哪些 Component ）
//! - Actor 调用 AddReceiver 时，检查类型及其父类是否在映射中，若存在直接 CreateComponentOnInstance 即可 (UGameFrameworkComponentManager::AddReceiverInternal)
//! 
//! 对于已有的 Actor 如何添加 Component
//! - 每个 GFA 在 Activating 的时候都会调用 OnGameFeatureActivating ，对于 UGameFeatureAction_AddComponents 最终会掉用 UGameFeatureAction_AddComponents::AddToWorld
//! - 里面调用 GFCM 的 UGameFrameworkComponentManager::AddComponentRequest
//!   - 添加 ReceiverClassToComponentClassMap
//!   - 依次获取 GameInstance 和 World ，并对其中所有 Actor 进行遍历和 CreateComponentOnInstance
//! 