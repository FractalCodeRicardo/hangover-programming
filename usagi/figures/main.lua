function _config()
  ---@type Usagi.Config
  return { name = "Game", game_id = "com.usagiengine.YOURGAMENAME" }
end

function _init()
  State = {}
end

function _update(dt)
end

function _draw(dt)
  gfx.clear(gfx.COLOR_BLACK)
  gfx.text("Hello, Usagi!", 10, 10, gfx.COLOR_WHITE)

  gfx.rect_fill(10, 30, 100, 50, gfx.COLOR_DARK_GREEN)
  gfx.circ_fill(200, 80, 50, gfx.COLOR_RED)
  gfx.tri_fill(100, 100, 50, 150, 150, 150, gfx.COLOR_DARK_PURPLE)
end
