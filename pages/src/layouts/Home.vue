<template>
  <Article>
    <H1 id="render-example">Markdown 渲染完整示例</H1>

    <P>
      这是一段用来检查行内混排的正文：<Strong>加粗</Strong>、<Em>斜体</Em>、<Del>删除线</Del>、<Mark>高亮</Mark>、<InlineCode text="const x = 1" />，以及<Link href="https://example.com">外链</Link>和<Link href="#render-example">内链</Link>。
    </P>

    <P>
      这一行检查硬换行：<Br />
      换行后的第二行，视觉上应该紧贴上一行，不产生段落间距。
    </P>

    <H2 id="section-code">第一节：代码块与 token</H2>

    <P>下面这段代码覆盖了全部 12 种 token 组件（含 Space）：</P>

    <CodeBlock lang="rust"><Comment text="// 计算并打印结果" /><Space :text="'\n'" /><KeyWord text="fn" /><Space :text="' '" /><Fun text="compute" /><Punct text="(" /><Variable text="x" /><Punct text=":" /><Space :text="' '" /><Type text="i32" /><Punct text=")" /><Space :text="' '" /><Operator text="->" /><Space :text="' '" /><Type text="i32" /><Space :text="' '" /><Punct text="{" /><Space :text="'\n    '" /><KeyWord text="let" /><Space :text="' '" /><Variable text="flag" /><Punct text=":" /><Space :text="' '" /><Type text="bool" /><Space :text="' '" /><Operator text="=" /><Space :text="' '" /><Constant text="true" /><Punct text=";" /><Space :text="'\n    '" /><KeyWord text="let" /><Space :text="' '" /><Plain text="tmp" /><Space :text="' '" /><Operator text="=" /><Space :text="' '" /><Number text="42" /><Space :text="' '" /><Operator text="+" /><Space :text="' '" /><Variable text="x" /><Punct text=";" /><Space :text="'\n    '" /><Fun text="println" /><Punct text="!" /><Punct text="(" /><String text='"result = {}"' /><Punct text="," /><Space :text="' '" /><Variable text="x" /><Punct text=")" /><Punct text=";" /><Space :text="'\n    '" /><Variable text="x" /><Space :text="' '" /><Operator text="+" /><Space :text="' '" /><Number text="1" /><Space :text="'\n'" /><Punct text="}" /></CodeBlock>

    <P>
      注意：上面 <InlineCode text="&lt;CodeBlock&gt;" /> 内部是紧凑输出，没有模板缩进，这样 <InlineCode text="textContent" /> 拿到的才是干净的原文，复制按钮才能正确工作。
    </P>

    <H3 id="section-quote">子节：引用块</H3>

    <Blockquote>
      <P>
        引用可以嵌套行内元素：<Strong>重点</Strong>、<Em>强调</Em>、<InlineCode text="inline()" />。
      </P>
    </Blockquote>

    <H4 id="section-h4">第四级标题</H4>

    <P>检查 H4 相对正文的层次。</P>

    <H2 id="section-list">第二节：列表</H2>

    <P>紧凑无序列表：</P>
    <Ul>
      <Li>列表项一</Li>
      <Li>列表项二，含 <InlineCode text="code" /></Li>
      <Li>列表项三</Li>
    </Ul>

    <P>有序列表，从 3 开始：</P>
    <Ol :start="3">
      <Li>第三步</Li>
      <Li>第四步</Li>
    </Ol>

    <P>嵌套列表 + 宽松列表（<InlineCode text="&lt;Li&gt;" /> 内包 <InlineCode text="&lt;P&gt;" />）：</P>

    <Ul>
      <Li>
        <P>外层项，宽松格式会包一层 P。</P>
        <Ul>
          <Li>内层 A</Li>
          <Li>内层 B</Li>
        </Ul>
      </Li>
      <Li><P>另一个外层项。</P></Li>
    </Ul>

    <H5 id="section-h5">第五级标题</H5>

    <P>H5 通常用于很深的层级，检查字号是否还清晰可读。</P>

    <H2 id="section-table">第三节：表格</H2>

    <Table>
      <Thead>
        <Tr>
          <Th align="left">语法</Th>
          <Th align="center">组件</Th>
          <Th align="right">状态</Th>
        </Tr>
      </Thead>
      <Tbody>
        <Tr>
          <Td align="left"><InlineCode text="# H1" /></Td>
          <Td align="center"><InlineCode text="&lt;H1&gt;" /></Td>
          <Td align="right">✅</Td>
        </Tr>
        <Tr>
          <Td align="left"><InlineCode text="> quote" /></Td>
          <Td align="center"><InlineCode text="&lt;Blockquote&gt;" /></Td>
          <Td align="right">✅</Td>
        </Tr>
        <Tr>
          <Td align="left"><InlineCode text="```rust" /></Td>
          <Td align="center"><InlineCode text="&lt;CodeBlock&gt;" /></Td>
          <Td align="right">✅</Td>
        </Tr>
        <Tr>
          <Td align="left"><InlineCode text="![alt](src)" /></Td>
          <Td align="center"><InlineCode text="&lt;Img&gt;" /></Td>
          <Td align="right">✅</Td>
        </Tr>
      </Tbody>
    </Table>

    <H6 id="section-h6">第六级标题</H6>

    <P>H6 是最后一级，检查它是否还能和正文区分开。</P>

    <P>图片组件：</P>

    <Img
      src="/images/demo.png"
      alt="示例图片"
      title="这是一个占位图，替换成你的资源路径"
    />

    <Hr />

    <P>分隔线之后的收尾段落，检查整篇文章的收束是否自然。</P>
  </Article>
</template>

<script setup lang="ts">
// 容器
import Article from "@/components/markdown/Article.vue";

// 块级
import H1 from "@/components/markdown/block/H1.vue";
import H2 from "@/components/markdown/block/H2.vue";
import H3 from "@/components/markdown/block/H3.vue";
import H4 from "@/components/markdown/block/H4.vue";
import H5 from "@/components/markdown/block/H5.vue";
import H6 from "@/components/markdown/block/H6.vue";
import P from "@/components/markdown/block/P.vue";
import Hr from "@/components/markdown/block/Hr.vue";
import Blockquote from "@/components/markdown/block/Blockquote.vue";
import Ul from "@/components/markdown/block/Ul.vue";
import Ol from "@/components/markdown/block/Ol.vue";
import Li from "@/components/markdown/block/Li.vue";
import CodeBlock from "@/components/markdown/block/CodeBlock.vue";
import Table from "@/components/markdown/block/Table.vue";
import Thead from "@/components/markdown/block/Thead.vue";
import Tbody from "@/components/markdown/block/Tbody.vue";
import Tr from "@/components/markdown/block/Tr.vue";
import Th from "@/components/markdown/block/Th.vue";
import Td from "@/components/markdown/block/Td.vue";

// 行内
import Strong from "@/components/markdown/inline/Strong.vue";
import Em from "@/components/markdown/inline/Em.vue";
import Del from "@/components/markdown/inline/Del.vue";
import Mark from "@/components/markdown/inline/Mark.vue";
import Br from "@/components/markdown/inline/Br.vue";
import InlineCode from "@/components/markdown/inline/InlineCode.vue";
import Link from "@/components/markdown/inline/Link.vue";
import Img from "@/components/markdown/inline/Img.vue";

// 代码 token
import KeyWord from "@/components/markdown/code/KeyWord.vue";
import Fun from "@/components/markdown/code/Fun.vue";
import Variable from "@/components/markdown/code/Variable.vue";
import Type from "@/components/markdown/code/Type.vue";
import String from "@/components/markdown/code/String.vue";
import Number from "@/components/markdown/code/Number.vue";
import Comment from "@/components/markdown/code/Comment.vue";
import Operator from "@/components/markdown/code/Operator.vue";
import Punct from "@/components/markdown/code/Punct.vue";
import Constant from "@/components/markdown/code/Constant.vue";
import Plain from "@/components/markdown/code/Plain.vue";
import Space from "@/components/markdown/code/Space.vue";
</script>
<!-- 

# Markdown 渲染组件备忘录

## 0. 三条总原则

1. **内容走 slot，元数据走 prop**。唯一例外是行内代码/代码 token——文本走 `text` prop 防插值。
2. **样式全在组件内 + `main.css` 变量**，编译器只输出结构，零 `<style>`。
3. **深色模式不写 `dark:`**，颜色一律走 `*-article-*` / `*-code-*` token，`.dark` 祖先自动切换。

## 1. 目录结构

```
src/components/markdown/
├── block/
│   ├── Article.vue                      容器（最外层必包）
│   ├── H1~H6.vue  P.vue  Hr.vue
│   ├── Blockquote.vue
│   ├── Ul.vue  Ol.vue  Li.vue
│   ├── CodeBlock.vue
│   └── Table.vue  Thead.vue  Tbody.vue  Tr.vue  Th.vue  Td.vue
├── inline/
│   ├── Strong.vue  Em.vue  Del.vue  Mark.vue  Br.vue
│   ├── InlineCode.vue
│   └── Link.vue  Img.vue
└── code/
    ├── KeyWord.vue  Fun.vue  Variable.vue  Type.vue
    ├── String.vue   Number.vue  Comment.vue
    ├── Operator.vue Punct.vue  Constant.vue  Plain.vue
    └── Space.vue
```

## 2. 容器 `Article`

| 项 | 说明 |
|---|---|
| 用法 | `<Article><slot /></Article>` |
| 职责 | 正文最大宽度、字体、行高、主色 |
| 关键类 | `max-w-article font-article leading-article px-8 py-12 mx-auto` |
| 约束 | **生成器最外层必包**，否则字体/行高全失效 |

## 3. 块级组件

### 标题 `H1` ~ `H6`

| 组件 | props | 视觉 |
|---|---|---|
| `H1` | `id?: string` | 居中、`text-h1`、下边框 `border-foreground` |
| `H2` | `id?: string` | 居中、宽 85%、`text-h2`、下边框 |
| `H3` | `id?: string` | 左边框 `border-l-2 border-foreground`、`pl-2.5` |
| `H4` | `id?: string` | 左对齐、`text-h4` |
| `H5` | `id?: string` | 同上，字号更小 |
| `H6` | `id?: string` | 同上，最小 |

- **`id` 必须透传**到原生标签，TOC 和 `#锚点` 依赖它。
- 生成器负责生成 slug（`标题1` → `标题1` 还是 `biao-ti-1`，自己定一个规则写死）。

### `P` 段落

```vue
<P><slot /></P>
```

- 无 props，无 `id`。
- `my-[0.8em]`，相邻段落 margin 折叠，不会双倍。

### `Hr` 分隔线

```vue
<Hr />
```

- 无 props。`<hr>` + `h-px bg-article-rule`，不是 `<div>`。
- **不要拿它当标题下划线**，标题线在 `H1/H2` 内部用 border 实现。

### `Blockquote` 引用块

```vue
<Blockquote><slot /></Blockquote>
```

- 无 props，内部通常包 `<P>`。
- 可嵌套，可含任意行内元素。

### `Ul` / `Ol` / `Li` 列表

| 组件 | props | 说明 |
|---|---|---|
| `Ul` | — | 无序号，`list-disc pl-8` |
| `Ol` | `start?: number` | `start="3"` 表示从 3 开始 |
| `Li` | — | 列表项 |

- **紧凑列表**：`<Li>文本</Li>`，不包 `<P>`。
- **宽松列表**：`<Li><P>文本</P></Li>`，`<P>` 提供段间距。
- 嵌套：`<Li>` 内再套 `<Ul>/<Ol>`，`pl-8` 自然缩进。
- 紧凑/宽松由 **mdast 的 `tight` 标记**决定，生成器判断，组件不感知。

### `CodeBlock` 代码块

| props | 类型 | 说明 |
|---|---|---|
| `lang` | `string?` | 语言标签，显示在头部，缺省不显示 |

- 内容 = **token 组件序列**，直接作为默认 slot。
- 组件内部已经套好 `<pre>`，slot 里**不要再包 `<pre>`**。
- 复制按钮读 `pre.textContent`，所以 slot 必须紧凑输出（见生成器约束）。
- 头部已有语言标签 + 复制按钮，生成器不用管。

### `Table` 系列

| 组件 | props | 说明 |
|---|---|---|
| `Table` | — | 外层容器，自带横向滚动 |
| `Thead` / `Tbody` | — | 无 props |
| `Tr` | — | 无 props |
| `Th` / `Td` | `align?: 'left' \| 'center' \| 'right'` | 列对齐；缺省走浏览器默认 |

- `align` 通过内联 style 实现，生成器从 md 表格的 `:---:` / `---:` 解析。
- 斑马纹第一版未实现，只有表头底色 + 边框。
- 结构固定：`Table > Thead > Tr > Th` 和 `Table > Tbody > Tr > Td`，生成器不要省 `<Tbody>`。

## 4. 行内组件

| 组件 | props | 内容 | 说明 |
|---|---|---|---|
| `Strong` | — | slot | `**x**` |
| `Em` | — | slot | `*x*` |
| `Del` | — | slot | `~~x~~` |
| `Mark` | — | slot | `==x==` |
| `Br` | — | 无 | 硬换行 |
| `InlineCode` | `text: string` | **prop** | 文本走 prop 防插值 |
| `Link` | `href: string` | slot | 内链/外链通吃 |
| `Img` | `src, alt?, title?` | 无 | 图片 |

**关键点**：

- `Strong / Em / Del / Mark / Link` **必须走 slot**，因为要支持嵌套（`**a *b* c**`、`[**加粗**](url)`）。
- `InlineCode` **必须走 prop**，防止 `` `{{ x }}` `` 被 Vue 解析。生成器做 HTML 实体转义。
- `Link` 自动判断外链（`^https?://`）加 `target="_blank" rel="noopener noreferrer"`，锚点和相对路径不加。
- `Img` 的 `src` 目前直接透传，Vite 资源处理留到生成器阶段决定（改写 `@/assets/...` 或 `new URL(..., import.meta.url)`）。

## 5. 代码 token 组件（11 + 1）

### 11 个颜色 token（同构）

| 组件 | syntect 典型 scope |
|---|---|
| `KeyWord` | `keyword.*` |
| `String` | `string.*` |
| `Comment` | `comment.*` |
| `Number` | `constant.numeric.*` |
| `Fun` | `entity.name.function`、`support.function` |
| `Variable` | `variable.*` |
| `Type` | `entity.name.type`、`storage.type` |
| `Constant` | `constant.language`（`true`/`false`/`null`） |
| `Operator` | `keyword.operator.*` |
| `Punct` | `punctuation.*` |
| `Plain` | 落不进任何 scope 的兜底（如局部变量名） |

**全部同构**：

```vue
<span class="text-code-xxx">…</span>
<script setup>defineProps<{ text: string }>()</script>
```

- **文本一律走 `text` prop**，不允许 slot。
- 生成器负责 HTML 实体转义：`&` `<` `>` `"` 四个字符。
- 组件名是**封闭集合**，换高亮引擎（syntect → tree-sitter）只改 Rust 映射表，组件不增不减。

### `Space` 空白

```vue
<template>{{ text }}</template>
<script setup>defineProps<{ text: string }>()</script>
```

- **无 span 包裹**，纯文本根，最轻。
- 承载所有空白：单空格、缩进、换行、连续空白块。
- 文本**必须用 `:text` 绑定**（含 `\n`），生成器统一用绑定表达式。
- 连续空白合并成一个 `<Space>`：`"\n    "` 不是两个组件。

## 6. 样式变量（main.css）

### 文章层

| 变量 | 用途 | 生成的类 |
|---|---|---|
| `--font-article` | 正文字体（拉丁在前、中文在后） | `font-article` |
| `--font-mono` | 代码字体 | `font-mono` |
| `--text-h1` ~ `--text-h4` | 标题字号 + 行高 | `text-h1` ~ `text-h4` |
| `--leading-article` | 正文行高 | `leading-article` |
| `--container-article` | 正文最大宽度 | `max-w-article` |
| `--color-article-link` | 链接色 | `text-article-link` |
| `--color-article-code-bg/fg` | 行内代码底/字 | `bg-article-code-bg` |
| `--color-article-mark-bg` | 高亮底 | `bg-article-mark-bg` |
| `--color-article-quote-bg/border` | 引用块 | `bg-article-quote-bg` |
| `--color-article-rule` | 分隔线、表格线 | `border-article-rule` |
| `--color-article-table-stripe` | 表格斑马纹 | `bg-article-table-stripe` |

### 代码层

`--code-keyword` `--code-string` `--code-comment` `--code-number` `--code-function` `--code-variable` `--code-type` `--code-constant` `--code-operator` `--code-punct` `--code-plain`，对应 `text-code-*`。

**两套变量都在 `:root`（light）和 `.dark`（dark）里各定义一次，`@theme inline` 里做 `--color-*: var(--*)` 映射。**

## 7. 生成器必须遵守的约束

这些是"生成物能跑"的硬要求，Rust 侧切词和拼串时逐条对照：

1. **最外层包 `<Article>`**，否则全局字体/行高失效。
2. **`<pre>` 内紧凑输出**：`<CodeBlock>` 开始标签后紧跟第一个 token，`</CodeBlock>` 前紧贴最后一个 token，中间**不能有模板缩进换行**。原因：`textContent` 会被模板空白污染。
3. **代码 token 文本走 `text` prop**，不允许把代码作为 slot 文本输出。含 `<` `>` `&` `"` 必须转义。
4. **空白用 `<Space :text="'...'" />`**，连续空白合并成一个，必须用 `:text` 绑定表达式。
5. **所有 prop 值做 HTML 实体转义**：`&` `<` `>` `"`。
6. **标题生成稳定 `id`**：slug 规则固定（建议保留中文或统一音译），**重复标题加 `-1` `-2` 后缀**。
7. **`<Ol>` 有 `start` 且不为 1 时才输出 `:start`**，避免无关 diff。
8. **`<Th>`/`<Td>` 的 `align` 缺省不输出**，只有 `:---:` / `---:` 才带。
9. **列表紧凑/宽松由 mdast `tight` 决定**：tight=true → `<Li>文本</Li>`；tight=false → `<Li><P>文本</P></Li>`。
10. **换行策略**：块级元素之间换行，行内组件与文本**尽量同行**（Prettier 会重排，但生成器输出尽量紧凑）。

## 8. 已知的坑

| 坑 | 现象 | 处理 |
|---|---|---|
| `v-pre` 与组件冲突 | 用了 `v-pre` 组件不渲染 | 弃用 `v-pre`，token 走 prop |
| 模板空白被 condense | `<pre>` 内 token 挤在一起 | 空白走 `<Space :text>` |
| Prettier 拆代码块 | 那行 `<CodeBlock>` 被拆成几十行 | 功能不受影响，可 `prettier-ignore` |
| `{{` 在代码里 | Vue 当插值解析 | 文本走 prop，不做 slot |
| `>` 在属性值 | 模板里合法，只在标签闭合位置解析 | 可不转义，但统一转 `&gt;` 更稳 |
| 紧凑/宽松列表样式不一致 | 列表间距跳变 | 生成器判断 `tight`，别在组件里统一 `my` |
| 标题下划线颜色 | resume 原版是正文色不是 rule 色 | `H1/H2` 用 `border-foreground` |
| 深色模式白底 | 漏了 `.dark` 变量覆盖 | 每个 `--article-*` / `--code-*` 两处都要定义 |
| `Li.vue` 误命名 `Il.vue` | import 找不到 | 已改名，注意别回退 |

## 9. 黄金样本

`ExampleArticle.vue` 是组件库的验收标准。Rust 生成器输出的第一篇文章和它 diff：

- 结构一致 = 生成逻辑正确
- 视觉一致 = 样式正确
- 复制结果一致 = 空白/token 处理正确

任何差异优先怀疑生成器，其次怀疑组件，最后才改样式变量。 -->