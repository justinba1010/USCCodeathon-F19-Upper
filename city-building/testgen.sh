SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"

mkdir -p input
mkdir -p output

pushd $SCRIPT_DIR

  for i in {3..15}
  do
    python3 ./mkin.py >> input/input$i;
    python3 ./solutions/solution.py < input/input$i >> output/output$i
  done

  for i in {0..3}
  do
    python3 ./solutions/solution.py < input/input$i >> output/output$i
  done
popd

rm -rf cases.zip
zip -r cases input output
