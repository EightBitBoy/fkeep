# pip install pytest
# pip install cli-test-helpers
# pytest -v --color=yes
import pytest

from cli_test_helpers import shell

BINARY_PATH = "target/debug/fkeep"


@pytest.fixture
def create_test_files():
    def _create(tmp_path, dirname="testdir", num_files=10):
        dir_path = tmp_path / dirname
        dir_path.mkdir()

        files = []
        for i in range(num_files):
            file = dir_path / f"file{i}.txt"
            file.touch()
            files.append(file)
        return dir_path, files

    return _create


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


# For playing around with pytest
def test_dir(tmp_path):
  dir = tmp_path / "foo"
  dir.mkdir()
  file_1 = dir / "1.txt"
  file_1.touch()
  file_2 = dir / "2.txt"
  file_2.touch()
  assert len(list(dir.iterdir())) == 2


# For playing around with pytest
def test_dir_2(tmp_path, create_test_files):
  dir_path, files = create_test_files(tmp_path)

  assert dir_path.exists()
  assert len(files) == 10
  assert len(list(dir_path.iterdir())) == 10
