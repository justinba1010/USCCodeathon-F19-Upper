SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"

mkdir -p in
mkdir -p out

pushd $SCRIPT_DIR

  for i in {1..10}
  do
    python3 ./mkin.py >> in/in$i;
    python3 ./solutions/solution.py < in/in$i >> out/out$i
  done

popd
