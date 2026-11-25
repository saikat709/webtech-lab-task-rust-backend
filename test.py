# to tell all endpoints
import requests

BASE_URL = "http://localhost:3000/tasks"

def test_get_all_tasks():
    response = requests.get(BASE_URL)
    assert response.status_code == 200
    tasks = response.json()
    assert isinstance(tasks, list)
    print("All tasks test passed!")

def test_create_task():
    payload = {
        "title": "Test Task",
        "description": "This is a test task."
    }
    response = requests.post(BASE_URL, json=payload)
    assert response.status_code == 201
    task = response.json()
    assert task["title"] == payload["title"]
    assert task["description"] == payload["description"]
    print("Create task test passed!")


def test_update_task():
    # First, create a task to update
    payload = {
        "title": "Task to Update",
        "description": "This task will be updated."
    }
    create_response = requests.post(BASE_URL, json=payload)
    assert create_response.status_code == 201
    task = create_response.json()
    task_id = task["id"]

    # Now, update the task
    update_payload = {
        "title": "Updated Task Title",
        "description": "This is the updated description."
    }
    update_response = requests.patch(f"{BASE_URL}/{task_id}", json=update_payload)
    assert update_response.status_code == 200
    updated_task = update_response.json()
    assert updated_task["message"] == "Task updated"
    print("Update task test passed!")

def test_delete_task():
    # First, create a task to delete
    payload = {
        "title": "Task to Delete",
        "description": "This task will be deleted."
    }
    create_response = requests.post(BASE_URL, json=payload)
    assert create_response.status_code == 201
    task = create_response.json()
    task_id = task["id"]

    # Now, delete the task
    delete_response = requests.delete(f"{BASE_URL}/{task_id}")
    assert delete_response.status_code == 204
    assert delete_response.text == ""
    print("Delete task test passed!")

if __name__ == "__main__":
    test_get_all_tasks()
    test_create_task()
    test_update_task()
    test_delete_task()
