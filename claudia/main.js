let img;
let identityShader;

function preload() {
  img = loadImage('assets/claudia.jpg');
  identityShader = loadShader('shader.vert', 'shader7.frag');
}

function setup() {
  createCanvas(img.width, img.height, WEBGL);
  noStroke();
  shader(identityShader);
  identityShader.setUniform('tex0', img);

}

function draw() {
    identityShader.setUniform('time', millis() / 1000.0);
    identityShader.setUniform('resolution', [width, height]);
  beginShape();
  // Top-left corner
  vertex(-1, -1, 0, 0);
  // Top-right
  vertex(1, -1, 1, 0);
  // Bottom-right
  vertex(1, 1, 1, 1);
  // Bottom-left
  vertex(-1, 1, 0, 1);
  endShape(CLOSE);
}
