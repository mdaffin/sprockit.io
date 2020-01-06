async function sendRequest(url, token, method) {
  const headers = token ? { "X-TOKEN": token } : {};
  const response = await fetch(`/api/game/maze/${url}`, { method, headers });

  if (!response.ok) {
    console.log("Error: " + (await response.text()));
  } else if (response.status == 204) {
    return;
  } else {
    return response.json();
  }
}

const post = async (url, token) => sendRequest(url, token, "POST");
const get = async (url, token) => sendRequest(url, token, "GET");

export class Maze {
  async moveTemplate(arg) {
    const map = await post(`move/${arg}`, this.token);
    parent.updateMaze(await get("map", this.token));
    return map;
  }

  async start() {
    this.token = (await post("start")).token;
    parent.updateMaze(await get("map", this.token));
  }

  async directions() {
    const map = await get("move", this.token);
    return map;
  }

  async moveUp() {
    return this.moveTemplate("up");
  }

  async moveDown() {
    return this.moveTemplate("down");
  }

  async moveLeft() {
    return this.moveTemplate("left");
  }

  async moveRight() {
    return this.moveTemplate("right");
  }
}
