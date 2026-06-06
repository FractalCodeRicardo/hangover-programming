function _config()
  ---@type Usagi.Config
  return { name = "Game", game_id = "com.usagiengine.YOURGAMENAME" }
end

function _init()
  State = {
    pos = {x = 100, y = 100},
    size = 30,
    sign = 1
  }
end

function _update(dt)

  if (State.size > 100 and State.sign > 0 ) then
    State.sign = -1
  end

  if (State.size < 30 and State.sign < 0) then
    State.sign = 1
  end

  State.size = State.size + dt * 50 * State.sign;

end

function _draw(dt)
  gfx.clear(gfx.COLOR_BLACK)
  gfx.text("Circle", 10, 10, gfx.COLOR_WHITE)

  local pos = State.pos
  gfx.circ_fill(pos.x, pos.y, State.size, gfx.COLOR_PINK)
end
