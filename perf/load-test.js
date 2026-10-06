// Simple load test for the web app. Run with: k6 run perf/load-test.js
// Override the target with: k6 run -e BASE_URL=http://host:8000 perf/load-test.js
import http from "k6/http";
import { check } from "k6";

const BASE_URL = __ENV.BASE_URL || "http://127.0.0.1:8000";

export const options = {
	stages: [
		{ duration: "10s", target: 20 }, // ramp up to 20 virtual users
		{ duration: "20s", target: 20 }, // hold
		{ duration: "5s", target: 0 }, // ramp down
	],
	thresholds: {
		http_req_failed: ["rate<0.01"], // less than 1% errors
		http_req_duration: ["p(95)<200"], // 95% of requests below 200ms
		checks: ["rate>0.99"],
	},
};

export default function () {
	const health = http.get(`${BASE_URL}/health`);
	check(health, {
		"health: status 200": (r) => r.status === 200,
		"health: status ok": (r) => r.json("status") === "ok",
	});

	const name = `user-${__VU}-${__ITER}`;
	const greet = http.post(`${BASE_URL}/greet`, JSON.stringify({ name }), {
		headers: { "Content-Type": "application/json" },
	});
	check(greet, {
		"greet: status 200": (r) => r.status === 200,
		"greet: correct message": (r) => r.json("message") === `Hello, ${name}!`,
	});
}
