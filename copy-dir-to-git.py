"""
これは　わたし用のプログラムだぜ☆つ（＾～＾）！
"""
import shutil

source = 'C:/Users/むずでょ/source/repos/piston-pixel-art-drawing-tool'
destination = 'C:/Users/むずでょ/Documents/GitHub/piston-pixel-art-drawing-tool'


def go():
    print('Trace   | Copy.')
    copy_dir('/.vscode')
    copy_dir('/@not-runtime')
    copy_dir('/piston-pixel-art',
             ignore=shutil.ignore_patterns('.git', 'target'))
    copy_file('/.gitignore')
    copy_file('/copy-dir-to-git.py')
    copy_file('/LICENSE')
    copy_file('/README.md')
    print('Trace   | Finished.')


def copy_dir(child_path, ignore=None):
    shutil.copytree(f'{source}{child_path}',
                    f'{destination}{child_path}', ignore=ignore)


def copy_file(child_path):
    shutil.copy2(f'{source}{child_path}', f'{destination}{child_path}')


go()
