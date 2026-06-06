function _config()
  ---@type Usagi.Config
  return { name = "Game", game_id = "com.usagiengine.YOURGAMENAME" }
end

function _init()
  State = {
    pos = { x = 100, y = 100},
    dir = { x = 1, y = 1}
  }
end

function _update(dt)

  State.pos.x += State.dir.x * dt * 200
  State.pos.y += State.dir.y * dt * 200

  if (State.pos.x > usagi.GAME_W or State.pos.x <= 0) then
    State.dir.x = State.dir.x * -1
  end

  if (State.pos.y > usagi.GAME_H or State.pos.y <=0) then
    State.dir.y = State.dir.y * -1
  end

end

function _draw(dt)
  gfx.clear(gfx.COLOR_BLACK)
  gfx.text("Bouncing", 10, 10, gfx.COLOR_WHITE)
  local pos = State.pos
  gfx.circ_fill(pos.x, pos.y, 20, gfx.COLOR_DARK_PURPLE)
end
