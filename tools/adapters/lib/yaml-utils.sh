#!/usr/bin/env bash
# YAML 工具函数库
# 提取 YAML frontmatter 字段、转换格式等

# 从文件中提取 YAML frontmatter（返回纯 YAML 文本）
extract_frontmatter() {
  local file="$1"
  sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d'
}

# 从文件中提取 Markdown body（不含 frontmatter）
extract_body() {
  local file="$1"
  sed '/^---$/,/^---$/d' "$file"
}

# 从 YAML 文本中获取字段值（简单实现，适用于单行字段）
get_yaml_field() {
  local yaml="$1"
  local field="$2"
  echo "$yaml" | grep "^${field}:" | head -1 | sed "s/^${field}:\s*//"
}

# 从 YAML 文本中获取列表字段（返回每行一个值）
get_yaml_list() {
  local yaml="$1"
  local field="$2"
  echo "$yaml" | sed -n "/^${field}:/,/^[^ ]/p" | grep -E "^\s+- " | sed 's/^\s*- //'
}

# 检查文件是否有 frontmatter
has_frontmatter() {
  local file="$1"
  head -1 "$file" | grep -q '^---$'
}

# 将 .studio 技能格式转换为 Claude Code 格式
# （metadata 字段提升为顶层）
convert_skill_to_claude() {
  local input_file="$1"
  local output_file="$2"
  local frontmatter
  local body

  frontmatter=$(extract_frontmatter "$input_file")
  body=$(extract_body "$input_file")

  # 生成 Claude Code frontmatter
  echo "---" > "$output_file"
  echo "$frontmatter" | grep "^name:" >> "$output_file"
  echo "$frontmatter" | grep "^description:" >> "$output_file"

  # 从 metadata 中提升字段
  local metadata
  metadata=$(echo "$frontmatter" | sed -n '/^metadata:/,/^[^ ]/p')
  echo "$metadata" | grep "^\s*model:" | sed 's/^\s*//' >> "$output_file" 2>/dev/null || true
  echo "$metadata" | grep "^\s*argument-hint:" | sed 's/^\s*//' >> "$output_file" 2>/dev/null || true
  echo "$metadata" | grep "^\s*user-invocable:" | sed 's/^\s*//' >> "$output_file" 2>/dev/null || true
  echo "$metadata" | sed -n '/^\s*allowed-tools:/,/^\s*[^-]/p' | sed 's/^\s*//' >> "$output_file" 2>/dev/null || true

  echo "---" >> "$output_file"
  echo "$body" >> "$output_file"
}

# 将 .studio 技能格式转换为 Windsurf 工作流格式
convert_skill_to_windsurf() {
  local input_file="$1"
  local output_file="$2"
  local name
  local description
  local body

  name=$(get_yaml_field "$(extract_frontmatter "$input_file")" "name")
  description=$(get_yaml_field "$(extract_frontmatter "$input_file")" "description")
  body=$(extract_body "$input_file")

  # 生成 Windsurf 工作流文件
  echo "# /${name} — ${description}" > "$output_file"
  echo "" >> "$output_file"
  echo "## 触发方式" >> "$output_file"
  echo "在 Cascade 对话框中输入 /${name} 触发此工作流。" >> "$output_file"
  echo "" >> "$output_file"
  echo "## 工作流内容" >> "$output_file"
  echo "$body" >> "$output_file"
}

# 将 .studio 技能格式转换为 Hermes 扩展格式
convert_skill_to_hermes() {
  local input_file="$1"
  local output_file="$2"
  local frontmatter
  local body
  local name
  local description

  frontmatter=$(extract_frontmatter "$input_file")
  body=$(extract_body "$input_file")
  name=$(get_yaml_field "$frontmatter" "name")
  description=$(get_yaml_field "$frontmatter" "description")

  # 生成 Hermes frontmatter
  echo "---" > "$output_file"
  echo "name: ${name}" >> "$output_file"
  echo "description: \"${description}\"" >> "$output_file"
  echo "version: 1.0.0" >> "$output_file"
  echo "platforms: [macos, linux, windows]" >> "$output_file"
  echo "metadata:" >> "$output_file"
  echo "  hermes:" >> "$output_file"
  echo "    tags: [\"${name}\"]" >> "$output_file"
  echo "    load_mode: on-demand" >> "$output_file"

  # 从源 metadata 中提取 model 和工具
  local metadata
  metadata=$(echo "$frontmatter" | sed -n '/^metadata:/,/^[^ ]/p')
  echo "$metadata" | grep "^\s*model:" | sed 's/^\s*model:/    model:/' >> "$output_file" 2>/dev/null || true
  echo "$metadata" | grep "^\s*user-invocable:" | sed 's/^\s*user-invocable:/    user-invocable:/' >> "$output_file" 2>/dev/null || true

  echo "---" >> "$output_file"
  echo "$body" >> "$output_file"
}