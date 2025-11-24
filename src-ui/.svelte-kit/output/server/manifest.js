export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["sprites/npc_mentor.png","sprites/player.png","sprites/player_down.png","sprites/player_left.png","sprites/player_right.png","sprites/player_up.png","tiles/door_locked.png","tiles/door_open.png","tiles/floor.png","tiles/floor_stone.png","tiles/floor_tech.png","tiles/terminal.png","tiles/void.png","tiles/wall.png","tiles/wall_top.png","ui/health_bar_bg.png","ui/health_bar_fill.png","ui/terminal_frame.png","ui/xp_bar_bg.png","ui/xp_bar_fill.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		client: {start:"_app/immutable/entry/start.DaDHVyVO.js",app:"_app/immutable/entry/app.BiqEFCqS.js",imports:["_app/immutable/entry/start.DaDHVyVO.js","_app/immutable/chunks/DbGOaw3d.js","_app/immutable/chunks/ftIhdwkj.js","_app/immutable/chunks/enAchYCB.js","_app/immutable/entry/app.BiqEFCqS.js","_app/immutable/chunks/ftIhdwkj.js","_app/immutable/chunks/C8CwK50t.js","_app/immutable/chunks/BjjLhWhE.js","_app/immutable/chunks/enAchYCB.js","_app/immutable/chunks/CJZO6bo7.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
