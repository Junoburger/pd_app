const Router = require("koa-router");
const router = new Router();
const axios = require("axios").default;
const Koa = require("koa");
const parser = require("koa-bodyparser");
const cors = require("@koa/cors");
const App = new Koa();
const port = 8000;

App.use(parser())
	.use(cors())
	.use(router.routes())
	.listen(port, () => {
		console.log(`🚀 Server listening http://127.0.0.1:${port}/ 🚀`);
	});

const get_random_word = async () => {
	const { data } = await axios.get("https://api.api-ninjas.com/v1/randomword");
	return data.word;
};

let random_N = () => Math.round(Math.random() * Math.random() * 100);

const artists = require("./artist.json");

const composition = (id, artist, desc, sounds, collaborators) => {
	// const title = await get_random_word();
	const _collaborators = [];
	return {
		id: id,
		artist: artist,
		title: "awesome composition",
		desc: desc,
		sounds: sounds,
		collaborators: collaborators,
	};
};

const get_compositions = () => {
	const iter_1 = artists.map((artist, i) => {
		let ix = random_N();
		let collaborators = artists[ix] !== undefined ? artists[ix] : artists[2];
		return composition(
			i,
			artist.name,
			"composition_name.reverse()",
			[1, 2, 3],
			[collaborators]
		);
	});
	const iter_2 = artists.map((artist, i) => {
		let ix = random_N();
		let collaborators = artists[ix] !== undefined ? artists[ix] : artists[2];
		return composition(
			i,
			artist.name,
			"composition_name.reverse()",
			[1, 2, 3],
			[collaborators]
		);
	});
	return iter_1.concat(iter_2);
};

router.get("/artists", (ctx) => {
	ctx.body = artists.map((artist) => {
		return {
			...artist,
			compositions: get_compositions().filter(
				(composition) => composition.artist === artist.name
			),
		};
	});

	ctx.status = 200;
});

router.get("/compositions", (ctx) => {
	ctx.body = get_compositions();
	ctx.status = 200;
});
