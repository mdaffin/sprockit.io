async function sendRequest(url, token, method) {
  const headers = token ? { "X-TOKEN": token } : {};
  const response = await fetch(`/api/game/maze/${url}`, { method, headers });

  if (!response.ok) {
    throw new Error(
      `bad response: ${response.status} ${
        response.statusText
      } ${await response.text()}`,
    );
  }

  if (response.status == 204) {
    return;
  }

  return response.json();
}

const post = async (url, token) => sendRequest(url, token, "POST");
const get = async (url, token) => sendRequest(url, token, "GET");

export class Maze {
  async start() {
    this.token = (await post("start")).token;
  }

  async moveUp() {
    await post("move/up", this.token);
  }

  async moveDown() {
    await post("move/down", this.token);
  }

  async moveLeft() {
    await post("move/left", this.token);
  }

  async moveRight() {
    await post("move/right", this.token);
  }

  async updateMaze() {
    parent.updateMaze(await get("map", this.token));
  }
}
