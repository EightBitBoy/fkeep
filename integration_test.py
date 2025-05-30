# pip install pytest
# pip install cli-test-helpers
# pytest -v --color=yes --basetemp=./tmp_tests
import os
import pytest
import time


from cli_test_helpers import shell


BINARY_PATH = "target/debug/fkeep"
DEFAULT_NUMBER_OF_FILES = 10


###########
# Helpers #
###########
def run(argumentsAndOptions = ""):
  return shell(BINARY_PATH + " " + argumentsAndOptions)


############
# Fixtures #
############
@pytest.fixture
def create_test_files():
    def _create(tmp_path, dirname="testdir", num_files=DEFAULT_NUMBER_OF_FILES):
        dir_path = tmp_path / dirname
        dir_path.mkdir()

        base_time = time.time() - (2 * 60 * 60)

        files = []
        for i in range(num_files):
            file_number = i + 1

            file = dir_path / f"file_{file_number:03d}.txt"
            file.write_text(f"{file_number:03d}")

            modification_time = base_time + (i * 60) # Modification time between files is one minute apart, simpler display with "ls -la"
            os.utime(file, (modification_time, modification_time))

            files.append(file)
        return dir_path, files

    return _create


####################
# Successful tests #
####################
def test_display_help():
  result = run("-h")
  assert result.exit_code == 0
  assert "Usage: fkeep" in result.stdout


#TODO: Parametrize!
def test_dry_run_3(tmp_path, create_test_files):
  dir_path, files = create_test_files(tmp_path)
  result = run(f"3 {str(dir_path)} -d")
  assert result.exit_code == 0
  assert len(list(dir_path.iterdir())) == DEFAULT_NUMBER_OF_FILES
  assert result.stdout != ""
  "Dry run; would delete files" in result.stdout


@pytest.mark.parametrize("number_of_files", range(1, 11))
def test_success_parameterized(tmp_path, create_test_files, number_of_files):
  dir_path, files = create_test_files(tmp_path)
  result = run(f"{number_of_files} {str(dir_path)}")
  assert result.exit_code == 0
  assert len(list(dir_path.iterdir())) == number_of_files
  assert result.stdout == ""


def test_larger_number_than_files(tmp_path, create_test_files):
  dir_path, files = create_test_files(tmp_path)
  result = run(f"11 {str(dir_path)}")
  assert result.exit_code == 0
  assert len(list(dir_path.iterdir())) == DEFAULT_NUMBER_OF_FILES
  assert result.stdout == ""


@pytest.mark.skip(reason="Not implemented")
def test_success_3_verbose():
  result = run("3 -v")
  assert result.exit_code == 0
  assert result.stdout == ""


@pytest.mark.skip(reason="Not implemented")
def test_success_3_verbose_verbose():
  result = run("3 -vv")
  assert result.exit_code == 0
  assert result.stdout == ""


#################
# Failing tests #
#################
def test_no_arguments():
  result = run()
  #TODO Correct exit code for this?
  assert result.exit_code == 2
  #TODO Print to stdout?
  assert "error: the following required arguments were not provided" in result.stderr


@pytest.mark.skip(reason="Not implemented")
def test_fail_zero(tmp_path, create_test_files):
  dir_path, files = create_test_files(tmp_path)
  result = run(f"0 {str(dir_path)}")
  assert result.exit_code == 1
  assert len(list(dir_path.iterdir())) == DEFAULT_NUMBER_OF_FILES
  assert result.stdout == "The number of files to keep must be greater than 0."


@pytest.mark.skip(reason="Not implemented")
def test_fail_negative(tmp_path, create_test_files):
  dir_path, files = create_test_files(tmp_path)
  result = run(f"-1 {str(dir_path)}")
  assert result.exit_code == 1
  assert len(list(dir_path.iterdir())) == DEFAULT_NUMBER_OF_FILES
  assert result.stdout == "The number of files to keep must be greater than 0."


##############
# Playground #
##############
def test_tmp_path_fixture(tmp_path):
  dir = tmp_path / "test"
  dir.mkdir()
  file_1 = dir / "one.txt"
  file_1.touch()
  file_2 = dir / "two.txt"
  file_2.touch()
  assert len(list(dir.iterdir())) == 2

def test_create_test_files_fixture(tmp_path, create_test_files):
  dir_path, files = create_test_files(tmp_path)
  assert dir_path.exists()
  assert len(files) == DEFAULT_NUMBER_OF_FILES
  assert len(list(dir_path.iterdir())) == DEFAULT_NUMBER_OF_FILES
