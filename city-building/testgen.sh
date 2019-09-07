SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"

mkdir -p input
mkdir -p output

pushd $SCRIPT_DIR

  for i in {3..15}
  do
    python3 ./mkin.py > input/input$i.txt;
    python3 ./solutions/solution.py < input/input$i.txt > output/output$i.txt
  done

  for i in {0..2}
  do
    python3 ./solutions/solution.py < input/input$i.txt > output/output$i.txt
  done
popd

rm -rf cases.zip
zip -r cases input output
