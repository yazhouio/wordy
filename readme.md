## 英语启蒙
### 1. 可输入简单单词，由 openai chatgpt 生成注释和例句。
### 2. 可选择例句，由 azure tts 生成语音。

### 技术栈
* 前端
    * rxjs: websocket 通信, 事件流处理
    * nextjs: 服务端渲染
    * nextui: 组件库
    * jotai: 状态管理, rxjs 与 react 结合
* 后端
    * axum: rust web 框架
### TODO

* 前端
  
- [x] access token 刷新
- [ ] 播放按钮自动切换中英文
- [ ] 播放界面
- [ ] angular 重写
- [ ] 聊天功能实现

* 后端
  
- [ ] 接口鉴权
- [ ] tts 带上语言参数
- [ ] grpc 支持
- [ ] 嵌入式数据库
- [ ] 配置重构
