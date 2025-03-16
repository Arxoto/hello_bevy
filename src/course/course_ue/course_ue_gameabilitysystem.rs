//! # GameAbilitySystem
//! 
//! 如何设计一个易于扩展的技能系统？
//! - 逻辑部分：
//!   - 技能的获取和释放
//!   - 触发判断的条件
//!   - Buff系统
//! - 视听部分：
//!   - 动作动画
//!   - 特效
//!   - 声效
//! - 数据部分：
//!   - 数值计算
//!   - 数值配置
//! - 网络联机同步（客户端预测）
//! 
//! 核心概念 Core Concepts
//! - UAbilitySystemComponent **ASC** 
//!   - 技能系统组件 拥有该组件的 Actor 才能管理释放技能
//! - UGameplayAbility **GA**
//!   - 表示一个能力逻辑（相比于技能更宽泛）
//! - UGameplayEffect **GE**
//!   - 游戏效果 属性修改或者动作效果的触发
//! - UGameplayCueNotify **GC**
//!   - 特效部分 爆炸或者燃烧等
//! - FGameplayAttribute **Attribute**
//!   - 描述游戏属性 生命攻击等 多个组合成一个 Set 挂在 Actor 上
//! - FGameplayTag **Tag**
//!   - 标签 主要用作解耦合 判断和搜索条件
//! - UGameplayTask **Task**
//!   - 异步操作 如播放动画蒙太奇结束后再释放技能
//! - FGameplayEventData **Event**
//!   - 事件通知
//! 
//! 
//! ## GameplayTags 游戏标签
//! 
//! 本身与 GAS 没关系，只不过被其重度使用。
//! 
//! 一个层次化的字符串标签 "A.B.C" ，整体所有的 Tag 构成一个 Tag 树，支持灵活地查询。
//! 
//! 
//! ## GameplayAttribute 游戏属性
//! 
//! 一个属性结构中有两个值（为了临时的Buff系统而设计）
//! - BaseValue 基础值、永久值
//! - CurrentValue 当前值、临时值、Buff生效后的结果（理解为 Actual 实际值比较好）
//! 
//! 注意 BaseValue 不应该当作最大值
//! - 最大值（血量上限）应该单独作为一个 Attribute 存在
//! - 对应的实时值（现有血量）也单独作为一个 Attribute 存在
//! 
//! 
//! ## GameplayEffect 技能效果 (data-only)
//! 
//! 决定一个技能的逻辑效果（以下简略）
//! - Duration Policy 持续方式（Instant瞬时、Infinite永久、HasDuration限时） 和 Expiration 过期处理
//! - Period 周期（do-once/while/do-while）
//! - Modifiers 属性修改器（所有 Attribute 的改变都通过 GE 生效）
//! - Application 应用条件（概率和 Tag 条件依赖）
//! - Granted Abilities 能力赋予（如一个点燃的状态，每秒对周围造成伤害）
//! - Stacking 堆叠 和 Overflow 溢出（同种效果层数叠加）
//! - Gameplay Cue Display 展示处理（特效音效）
//! 
//! 
//! ## GameplayCue 技能特效
//! 
//! 一般通过 GE 触发，也可手动触发，分为一次性和持久
//! 
//! 
//! ## GameAbility 游戏能力（技能）
//! 
//! 能力是很广义的抽象，一般认为是专门触发一件事情，而非基础移动等一直在做的事情
//! 
//! - Tags 配置
//!   - 是否拥有该能力
//!   - 是否取消
//!   - 是否格挡
//! - Costs 消耗
//! - Cooldowns 冷却
//! - Trigger 触发机制
//! 
//! 
//! ## GameplayTask 异步任务
//! 
//! 如等待蒙太奇播放结束、直接一段时间后触发
//! 
//! 
//! ## GameplayEvent 游戏事件
//! 
//! 事件可靠 Tag 区分，并且可携带 Payload 数据
//! 
//! 
//! ## AbilitySystemComponent 游戏技能组件
//! 
//! ASC 负责协调其他部件，是技能系统运行的核心
//! 
//! 