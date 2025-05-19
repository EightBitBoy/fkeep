# pip install pytest
# pip install cli-test-helpers
# pytest -v --color=yes


from cli_test_helpers import shell

BINARY_PATH = "target/debug/fkeep"

def run(argumentsAndOptions = ""):
  return shell(BINARY_PATH + " " + argumentsAndOptions)


def test_no_arguments():
  result = run()
  #TODO Correct exit code for this?
  assert result.exit_code == 2
  #TODO Print to stdout?
  assert "error: the following required arguments were not provided" in result.stderr

def test_success():
  result = run("3")
  assert result.exit_code == 0
  assert result.stdout == ""
