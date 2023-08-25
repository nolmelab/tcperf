# git revert commit 

특정 로컬 커밋을 revert 하는 방법을 찾는다. 150MB가 넘는 로그 파일을
커밋해서 git으로 push가 안 된다. 

# This will destroy any local modifications.
# Don't do it if you have uncommitted work you want to keep.
git reset --hard 0d1d7fc32

# Alternatively, if there's work to keep:
git stash
git reset --hard 0d1d7fc32
git stash pop
# This saves the modifications, then reapplies that patch after resetting.
# You could get merge conflicts if you've modified things which were
# changed since the commit you reset to.

git reflog 로 커밋들을 볼 수 있다. 

