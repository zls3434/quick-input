<!-- Software Engineering Studios -->
# 技能参考

本文件列出 Software Engineering Studios 中配置的 79 个技能（Skill），按开发阶段分类，并标注模型层级分配。所有技能定义源文件位于 `.studio/skills/` 目录，通过同步脚本输出到各平台。

## 1. 模型层级分配

| 模型层级 | 适用技能 | 分配原则 |
| --- | --- | --- |
| **Haiku**（快速轻量） | `help`、`sprint-status`、`story-readiness`、`scope-check`、`changelog`、`patch-notes`、`onboard`、`project-stage-detect` | 只读检查、状态查询、简单格式化 |
| **Opus**（深度推理） | `gate-check`、`review-all-srs`、`architecture-review`、`asset-review` | 多文档综合、高风险决策、复杂审查 |
| **Sonnet**（标准能力） | 其余所有技能 | 实现与设计任务的默认选择 |

## 2. 按阶段分类的技能清单

### 2.1 入门导航（5 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/start` | Sonnet | 引导式上手流程，检测项目阶段并推荐起点 |
| `/help` | Haiku | 查看所有可用命令和 Agent 列表 |
| `/onboard` | Haiku | 新成员入职引导，展示项目结构和规范 |
| `/project-stage-detect` | Haiku | 自动检测当前项目所处阶段 |
| `/adopt` | Sonnet | 将现有项目纳入工作室管理体系 |

### 2.2 需求阶段（8 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/brainstorm` | Sonnet | 产品头脑风暴，由产品总监引导产生创意 |
| `/setup-stack` | Sonnet | 技术栈配置，生成 `technical-preferences.md` |
| `/product-concept` | Sonnet | 生成产品概念文档，定义产品定位和核心功能 |
| `/map-modules` | Sonnet | 将产品概念拆解为系统模块，生成系统索引 |
| `/requirement-spec` | Sonnet | 为各模块编写详细需求规格说明，可重复执行 |
| `/quick-spec` | Sonnet | 快速需求规格，轻量版需求文档生成 |
| `/review-all-srs` | Opus | 全量需求规格审查，跨文档一致性检查 |
| `/consistency-check` | Sonnet | 需求一致性检查，检测文档间矛盾 |

### 2.3 架构设计阶段（7 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/design-review` | Sonnet | 设计审查，检查设计文档完整性和合理性 |
| `/create-architecture` | Sonnet | 创建架构文档，定义系统架构和模块划分 |
| `/architecture-decision` | Sonnet | 记录架构决策（ADR），至少生成 3 条 |
| `/architecture-review` | Opus | 架构审查，由首席架构师对架构进行全面审查 |
| `/create-control-manifest` | Sonnet | 创建控制清单，列出受控文件和变更流程 |
| `/propagate-design-change` | Sonnet | 传播设计变更到所有受影响的 Agent 和文档 |
| `/reverse-document` | Sonnet | 逆向文档生成，为现有代码生成文档 |

### 2.4 技术设计阶段（5 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/design-system-spec` | Sonnet | 定义设计系统，包括设计令牌和组件库规格 |
| `/ux-design` | Sonnet | 为各模块创建 UX 设计文档，至少 3 个 |
| `/ux-review` | Sonnet | UX 审查，检查 UX 设计一致性和可用性 |
| `/api-spec` | Sonnet | 定义 API 接口规格，包括端点和请求/响应格式 |
| `/accessibility-requirements` | Sonnet | 定义无障碍需求，确保符合 WCAG 标准 |

### 2.5 预开发阶段（7 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/prototype` | Sonnet | 制作快速原型验证关键交互和技术可行性 |
| `/build-mvp` | Sonnet | 构建最小可行产品验证核心价值假设 |
| `/create-epics` | Sonnet | 将需求转化为史诗，定义粗粒度工作项 |
| `/create-stories` | Sonnet | 将史诗拆解为可执行的用户故事 |
| `/test-setup` | Sonnet | 搭建测试基础设施，包括框架和 mock 服务 |
| `/sprint-plan` | Sonnet | 规划 Sprint 内容，分配故事并估算工作量 |
| `/estimate` | Sonnet | 工作量估算，为故事和史诗提供时间预估 |

### 2.6 开发阶段（11 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/dev-story` | Sonnet | 开发单个用户故事，实现功能并编写测试 |
| `/code-review` | Sonnet | 对已完成代码进行审查，确保符合编码规范 |
| `/story-readiness` | Haiku | 检查故事是否满足开发就绪条件 |
| `/story-done` | Sonnet | 确认故事完成，验证验收标准和测试证据 |
| `/sprint-status` | Haiku | 生成当前 Sprint 状态报告 |
| `/scope-check` | Haiku | 检查当前工作是否超出 Sprint 范围 |
| `/bug-report` | Sonnet | 记录发现的缺陷，包括复现步骤和优先级 |
| `/bug-triage` | Sonnet | 缺陷分诊，评估缺陷优先级和分配责任人 |
| `/hotfix` | Sonnet | 热修复流程，快速修复生产环境问题 |
| `/qa-plan` | Sonnet | 制定 Sprint 级别的 QA 计划 |
| `/retrospective` | Sonnet | Sprint 回顾，总结经验教训和改进项 |

### 2.7 测试优化阶段（7 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/perf-profile` | Sonnet | 对系统进行性能分析和瓶颈定位 |
| `/security-audit` | Sonnet | 执行安全审计，检查漏洞和合规性 |
| `/tech-debt` | Sonnet | 评估技术债务并制定偿还计划 |
| `/smoke-check` | Sonnet | 执行冒烟测试，验证核心功能可用性 |
| `/soak-test` | Sonnet | 执行耐久测试，检测内存泄漏和性能衰减 |
| `/regression-suite` | Sonnet | 运行回归测试套件，确保未引入新缺陷 |
| `/test-evidence-review` | Sonnet | 审查所有测试证据，确保覆盖验收标准 |

### 2.8 发布部署阶段（6 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/gate-check` | Opus | 执行发布门禁检查，验证所有质量条件 |
| `/release-checklist` | Sonnet | 生成并执行发布检查清单 |
| `/launch-checklist` | Sonnet | 生成上线检查清单，包括部署和回滚方案 |
| `/changelog` | Haiku | 根据提交记录自动生成变更日志 |
| `/patch-notes` | Haiku | 生成面向用户的补丁说明 |
| `/day-one-patch` | Sonnet | 首日补丁流程，处理上线后紧急问题 |

### 2.9 团队编排（7 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/team-frontend` | Sonnet | 前端团队编排，协调前端相关 Agent 协作 |
| `/team-backend` | Sonnet | 后端团队编排，协调后端相关 Agent 协作 |
| `/team-qa` | Sonnet | QA 团队编排，协调测试相关 Agent 协作 |
| `/team-release` | Sonnet | 发布团队编排，协调发布相关 Agent 协作 |
| `/team-polish` | Sonnet | 打磨团队编排，协调 UI/UX 优化相关 Agent |
| `/team-security` | Sonnet | 安全团队编排，协调安全审计相关 Agent |
| `/team-devops` | Sonnet | DevOps 团队编排，协调基础设施相关 Agent |

### 2.10 工具类（14 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/content-audit` | Sonnet | 内容审计，检查文档和代码内容质量 |
| `/test-flakiness` | Sonnet | 测试抖动检测，识别不稳定测试 |
| `/skill-test` | Sonnet | 技能测试，验证技能定义变更是否正确 |
| `/skill-improve` | Sonnet | 技能改进，优化现有技能定义 |
| `/user-test-report` | Sonnet | 用户测试报告，汇总用户测试结果 |
| `/audit-docs` | Sonnet | 文档审计，检查文档完整性和一致性 |
| `/test-helpers` | Sonnet | 测试辅助库与工厂函数生成 |
| `/localize` | Sonnet | 国际化扫描、提取与验证 |
| `/milestone-review` | Sonnet | 里程碑审查，检查里程碑进度 |
| `/create-asset` | Sonnet | 动态扩展统一入口，引导选择创建 Agent/Skill/Rule |
| `/create-agent` | Sonnet | 创建新专家 Agent，六阶段工作流确保合规 |
| `/create-skill` | Sonnet | 创建新技能，符合 Agent Skills 开放标准 |
| `/create-rule` | Sonnet | 创建新路径规则，确保不与现有规则冲突 |
| `/asset-review` | Opus | 资产审核，总监层审核新增资产提案 |

### 2.11 平台工具（2 个）

| 命令 | 模型 | 说明 |
| --- | --- | --- |
| `/sync-platforms` | Sonnet | 平台同步，将 .studio/ 规范源同步到所有平台 |
| `/platform-check` | Haiku | 平台配置检查，检查各平台配置一致性 |

## 3. 技能总数统计

| 阶段分类 | 数量 |
| --- | --- |
| 入门导航 | 5 |
| 需求阶段 | 8 |
| 架构设计阶段 | 7 |
| 技术设计阶段 | 5 |
| 预开发阶段 | 7 |
| 开发阶段 | 11 |
| 测试优化阶段 | 7 |
| 发布部署阶段 | 6 |
| 团队编排 | 7 |
| 工具类 | 14 |
| 平台工具 | 2 |
| **合计** | **79** |

## 4. 技能定义格式

每个技能在 `.claude/skills/` 目录下有一个对应的 `.md` 文件，格式如下：

```markdown
---
name: skill-name
command: /skill-name
model: sonnet
phase: development
required: true
repeatable: false
artifact:
  glob: "path/to/artifact"
  pattern: "path/to/artifact"
  min_count: 1
---

# 技能名称

## 用途
[技能用途说明]

## 执行流程
1. [步骤1]
2. [步骤2]

## 传入数据
- [参数说明]

## 输出
- [产出物说明]
```

## 5. 技能与命令的对应关系

- 每个技能对应一个斜杠命令
- 斜杠命令是技能的触发入口
- 技能定义了命令的完整行为
- 技能可调用一个或多个 Agent 执行

## 6. 技能变更管理

- 技能文件变更时，`validate-skill-change.sh` Hook 会发出提示
- 技能变更后应运行 `/skill-test` 验证
- 技能改进可使用 `/skill-improve` 命令
- 技能变更需通过 `chief-architect` 审批

### 6.1 动态扩展机制

Software Engineering Studios 支持根据项目需求动态新增 Agent、技能和规则：

- **创建新资产**：使用 `/create-asset`（统一入口）、`/create-agent`、`/create-skill`、`/create-rule`
- **资产审核**：使用 `/asset-review`，由 chief-architect 执行 CA-ASSET 门禁审核
- **六阶段工作流**：提案→草稿→审核→批准→注册→同步
- **注册与追踪**：所有新增资产注册到 `asset-registry.yaml`，变更记录到 `asset-changelog.md`
- **权限矩阵**：总监层和部门负责人层可提案，chief-architect 审核，用户最终批准

完整规范详见 `docs/extension-mechanism.md`。