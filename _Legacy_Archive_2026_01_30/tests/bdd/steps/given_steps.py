from behave import given
import subprocess
import step_utils

@given('the processor docker container is stopped')
def step_stop_container(context):
    try:
        step_utils.run_manage_cmd(["stop"])
    except subprocess.CalledProcessError as e:
        print(f"Warning: Failed to stop container: {e}")

@given('the processor is running')
def step_impl_processor_running(context):
    status = step_utils.api_call("status")
    assert status is not None, "Processor API is unreachable"

@given('the "{dir_name}" directory is empty')
def step_impl_dir_empty_given(context, dir_name):
    step_utils.impl_dir_empty(context, dir_name)

@given('I have enabled auto-process')
def step_impl_enable_auto_given(context):
    step_utils.impl_enable_auto(context)

@given('I have disabled auto-process')
def step_impl_disable_auto_given(context):
    step_utils.impl_disable_auto(context)
