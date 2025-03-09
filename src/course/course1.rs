//! # course 1: Hello ECS [`main`]
//!
//! ECS 设计模式：低耦合、数据驱动、缓存优化、高并发
//!
//! - Entity
//!   - 实体 具有实际意义的实例化对象 仅是一个标识符 通过关联各个组件实现功能
//! - Component
//!   - 组件 实体具有的不同状态数据等 组件之间相互独立
//! - System
//!   - 系统 处理组件的逻辑 因为组件独立性 天然具备并发性
//! - Plugin
//!   - 插件 一个独立的逻辑，打包了实体、组件、系统等
//!   - Bevy 中所有模块都以插件的形式实现 如 UI 渲染 等等
//! - Resource
//!   - 资源 全局唯一、全局共享的数据
//! 
//! todo
//! 
//! - 与 UE 的设计框架 GameFeatures(or the earlier: GamePlay) 对比，各个模块分属哪里

use bevy::prelude::*;

/// 打开一个什么都没有的窗口 （ DefaultPlugins 引入的窗口相关插件）
/// 
/// 所有的 hello 都会无限循环 （ DefaultPlugins 引入了事件循环）
#[allow(dead_code)]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        // or just
        // .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

/// 插件 每一帧 say hello
struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        // 可以保证执行顺序
        app.add_systems(Update, (greet_people, update_people).chain());
        // 使用资源的 say hello
        app.add_plugins(HelloWithTimePlugin);
    }
}

/// 拥有这个组件表示实体是一个人
#[derive(Component)]
struct Person;

/// 拥有这个组件表示实体具有名称
#[derive(Component)]
struct Name(String);

/// 初始化 system 通过命令队列对世界执行结构更改
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

/// 查询 system 对所有人 say hello
fn greet_people(query: Query<&Name, With<Person>>) {
    println!("=========");
    for name in &query {
        println!("hello {}!", name.0);
    }
}

/// 可变查询 system 修改某个人的名称
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

/// 插件 每隔 2s say hello
struct HelloWithTimePlugin;

impl Plugin for HelloWithTimePlugin {
    fn build(&self, app: &mut App) {
        // 2s 反复触发
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        // 可以保证执行顺序
        app.add_systems(Update, greet_people_with_timer);
    }
}

/// 资源 全局唯一数据 用于判断是否满足 say hello 的条件
#[derive(Resource)]
struct GreetTimer(Timer);

/// 使用资源判断是否 say hello
fn greet_people_with_timer(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        println!("=========");
        println!("hello, {} people!", query.iter().count());
    }
}
