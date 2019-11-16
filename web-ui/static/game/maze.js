export class Maze {
  async start() {
    const response = await fetch("/api/game/maze/start", {
      method: "POST",
    });

    if (!response.ok) {
      throw new Error(
        `bad response: ${response.status} ${
          response.statusText
        } ${response.text()}`,
      );
    }

    this.token = (await response.json()).token;
  }
}
