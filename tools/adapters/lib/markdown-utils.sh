#!/usr/bin/env bash
# Markdown 工具函数库
# Markdown 文件的处理、提取、转换

# 提取 Markdown 文件中的标题
extract_headings() {
  local file="$1"
  grep -E '^#{1,6}\s' "$file"
}

# 从 Markdown 标题中提取标题文本（去除 # 前缀）
heading_text() {
  local heading_line="$1"
  echo "$heading_line" | sed 's/^#\+\s*//'
}

# 从 Markdown 标题中提取级别
heading_level() {
  local heading_line="$1"
  echo "$heading_line" | grep -o '^#\+' | wc -c
}

# 生成目录（从 Markdown 标题）
generate_toc() {
  local file="$1"
  local headings
  local level
  local text
  local indent

  headings=$(extract_headings "$file")
  while IFS= read -r line; do
    level=$(heading_level "$line")
    text=$(heading_text "$line")
    indent=$(printf '%*s' $((level * 2 - 2)) '')
    echo "${indent}- ${text}"
  done <<< "$headings"
}

# 提取文件头注释（<!-- ... -->）
extract_header_comment() {
  local file="$1"
  sed -n '/^<!--.*-->$/p' "$file" | head -1
}

# 添加文件头注释
add_header_comment() {
  local file="$1"
  local comment="$2"
  # 检查是否已有文件头
  if ! head -1 "$file" | grep -q '^<!--'; then
    # 在文件开头插入注释
    sed -i "1i<!-- ${comment} -->" "$file"
  fi
}

# 统计文件行数
count_lines() {
  local file="$1"
  wc -l < "$file" | tr -d ' '
}

# 统计 Markdown 文件中的代码块数量
count_code_blocks() {
  local file="$1"
  grep -c '```' "$file" 2>/dev/null || echo 0
}

# 统计 Markdown 文件中的表格行数
count_table_rows() {
  local file="$1"
  grep -cE '^\|.*\|$' "$file" 2>/dev/null || echo 0
}

# 将规则内容从 .studio 格式转换为 Cursor .mdc 格式
convert_rule_to_mdc() {
  local input_file="$1"
  local output_file="$2"
  local rule_name
  local paths_glob
  local description

  rule_name=$(basename "$input_file" .md)
  paths_glob=$(get_yaml_field "$(extract_frontmatter "$input_file")" "paths")

  # 生成描述
  description=$(extract_body "$input_file" | head -1 | sed 's/^#\+\s*//')

  # 生成 .mdc frontmatter
  echo "---" > "$output_file"
  echo "description: \"${description}\"" >> "$output_file"

  # 判断是否为 Always 类型
  if [ "$rule_name" = "00-overview" ] || [ "$rule_name" = "01-coordination" ] || [ "$rule_name" = "02-coding-standards" ]; then
    echo "globs: \"\"" >> "$output_file"
    echo "alwaysApply: true" >> "$output_file"
  else
    echo "globs: \"${paths_glob}\"" >> "$output_file"
    echo "alwaysApply: false" >> "$output_file"
  fi

  echo "---" >> "$output_file"
  extract_body "$input_file" >> "$output_file"
}

# 将规则内容内联到 Windsurf 格式
convert_rule_to_windsurf() {
  local input_file="$1"
  local rule_name
  local body

  rule_name=$(basename "$input_file" .md)
  body=$(extract_body "$input_file")

  echo "" >> "$2"
  echo "---" >> "$2"
  echo "# ${rule_name}" >> "$2"
  echo "$body" >> "$2"
}