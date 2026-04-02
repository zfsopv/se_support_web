#!/usr/bin/env python3
import os
import re
import subprocess
import shutil

GERRIT_HOST = "zetao.zhang@gerrit-ai.sophgo.vip:29418"
REPOS = ['sophgo-develop-docs', 'libsophon', 'libsophav', 'tpu-mlir', 'sophon-sail', 'sophon-testhub']

def clean_env(workspace, output_dir):
    print("清理输出目录...")
    if os.path.exists(output_dir):
        shutil.rmtree(output_dir)
    os.makedirs(output_dir, exist_ok=True)
    os.makedirs(workspace, exist_ok=True)

def parse_version_file(version_file):
    print("解析版本文件...")
    repo_branch = {}
    repo_commit = {}
    
    if not os.path.exists(version_file):
        print(f"  警告: 版本文件不存在 {version_file}")
        return repo_branch, repo_commit
    
    with open(version_file, 'r', encoding='utf-8') as f:
        current_repo = None
        for line in f:
            line = line.strip()
            if line.startswith('module:') and '(' in line:
                start = line.index('module:') + 7
                end = line.index('(')
                current_repo = line[start:end].strip()
                branch_start = line.index('(') + 1
                branch_end = line.index(')')
                branch = line[branch_start:branch_end].strip()
                repo_branch[current_repo] = branch
            elif line.startswith('commit'):
                commit = line.replace('commit', '').strip()
                if current_repo and commit:
                    repo_commit[current_repo] = commit
    
    return repo_branch, repo_commit

def run_cmd(cmd, cwd=None, check=True):
    print(f"  执行: {cmd}")
    result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    if result.returncode != 0:
        if check:
            print(f"  警告: 命令执行失败 - {result.stderr}")
        else:
            print(f"  {result.stderr}")
    return result

def clone_and_checkout(repo, branch, commit, workspace):
    repo_dir = os.path.join(workspace, repo)
    repo_url = f"ssh://{GERRIT_HOST}/{repo}"
    
    print(f"=== 处理仓库: {repo} (分支: {branch}) ===")
    
    if os.path.exists(repo_dir):
        print("  仓库已存在，使用本地仓库")
        run_cmd("git clean -fd", cwd=repo_dir, check=False)
        run_cmd("git reset --hard", cwd=repo_dir, check=False)
        run_cmd("git fetch origin", cwd=repo_dir, check=False)
    else:
        print("  克隆仓库")
        run_cmd(f"git clone -b {branch} {repo_url} {repo_dir}")
    
    if commit:
        print(f"  切换到 commit: {commit}")
        run_cmd(f"git checkout {commit}", cwd=repo_dir)
    else:
        print("  使用分支 HEAD")
        run_cmd(f"git checkout origin/{branch}", cwd=repo_dir)

def is_chinese_doc(text, file_path):
    chinese_pattern = re.compile(r'[\u4e00-\u9fff]')
    chinese_chars = chinese_pattern.findall(text)
    
    if not chinese_chars:
        return False
    
    total_chars = len(text)
    chinese_ratio = len(chinese_chars) / total_chars
    
    if chinese_ratio >= 0.05:
        return True
    
    return False

def merge_rst_files(output_file, src_dir, repo):
    if not os.path.isdir(src_dir):
        print(f"  警告: 源目录不存在 {src_dir}")
        return 0

    print(f"=== 合并目录: {src_dir} 到 {output_file} ===")

    rst_files = []
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.rst'):
                full_path = os.path.join(root, file)
                if repo == "sophgo-develop-docs":
                    if not ("sophgo-develop-docs/cvitek/athena2-img" in full_path or "sophgo-develop-docs/multimedia" in full_path):
                        continue
                rst_files.append(full_path)

    rst_files.sort()

    valid_count = 0
    skip_small = 0
    skip_no_chinese = 0

    with open(output_file, 'w', encoding='utf-8') as out:
        for rst_file in rst_files:
            try:
                with open(rst_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                lines = content.split('\n')
                line_count = len(lines)

                if line_count < 100:
                    skip_small += 1
                    continue

                if not is_chinese_doc(content, rst_file):
                    skip_no_chinese += 1
                    continue

                out.write(f"\n")
                out.write(f"{'='*40}\n")
                out.write(f"文件: {rst_file}\n")
                out.write(f"{'='*40}\n")
                out.write(f"\n")
                out.write(content)
                out.write(f"\n\n")
                valid_count += 1

            except Exception as e:
                print(f"  警告: 读取文件失败 {rst_file}: {e}")
                continue

    print(f"  跳过(行数<50): {skip_small}, 跳过(无中文): {skip_no_chinese}, 合并: {valid_count}")
    return valid_count

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    version_file = os.path.join(script_dir, 'VERSION_FILE')
    workspace = os.path.join(script_dir, 'a2_ws')
    output_dir = os.path.join(script_dir, 'a2_doc_out')

    clean_env(workspace, output_dir)
    repo_branch, repo_commit = parse_version_file(version_file)

    for repo in REPOS:
        branch = repo_branch.get(repo, 'master')
        commit = repo_commit.get(repo, '')
        clone_and_checkout(repo, branch, commit, workspace)

    print("\n=== 开始合并中文RST源文档 ===\n")

    total_files = 0
    for repo in REPOS:
        src_dir = os.path.join(workspace, repo)
        output_file = os.path.join(output_dir, f"{repo}.rst")
        count = merge_rst_files(output_file, src_dir, repo)
        total_files += count

    print(f"\n=== 合并完成 ===")
    print(f"总共合并了 {total_files} 个文件")
    print(f"输出目录: {output_dir}")

    for f in sorted(os.listdir(output_dir)):
        file_path = os.path.join(output_dir, f)
        if os.path.isfile(file_path):
            size = os.path.getsize(file_path)
            print(f"  {f}: {size} bytes")

if __name__ == '__main__':
    main()
