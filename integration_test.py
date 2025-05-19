# pip install pytest
# pip install cli-test-helpers
# pytest -v --color=yes
import pytest

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

def test_help():
  result = run("-h")
  assert result.exit_code == 0
  assert "Usage: fkeep" in result.stdout

@pytest.mark.skip(reason="Not implemented")
def test_success_3():
  result = run("3")
  assert result.exit_code == 0
  assert result.stdout == ""

@pytest.mark.skip(reason="Not implemented")
def test_success_5():
  result = run("5")
  assert result.exit_code == 0
  assert result.stdout == ""

@pytest.mark.skip(reason="Not implemented")
def test_success_3_verbose():
  result = run("3 -v")
  assert result.exit_code == 0
  assert result.stdout == ""

@pytest.mark.skip(reason="Not implemented")
def test_success_5_verbose():
  result = run("5 -v")
  assert result.exit_code == 0
  assert result.stdout == ""

@pytest.mark.skip(reason="Not implemented")
def test_success_3_verbose_verbose():
  result = run("3 -vv")
  assert result.exit_code == 0
  assert result.stdout == ""

@pytest.mark.skip(reason="Not implemented")
def test_success_5_verbose_verbose():
  result = run("5 -vv")
  assert result.exit_code == 0
  assert result.stdout == ""

@pytest.mark.skip(reason="Not implemented")
def test_dry_run():
  pass
