import http from "k6/http";
import { check } from "k6";

export const options = {
    scenarios: {
        find_max_rps: {
            executor: "ramping-arrival-rate",
            startRate: 1000,
            timeUnit: "1s",
            stages: [
                { duration: "1m", target: 5_000 },
                { duration: "1m", target: 10_000 },
                { duration: "1m", target: 20_000 },
                { duration: "1m", target: 50_000 },
                { duration: "1m", target: 100_000 },
            ],
            preAllocatedVUs: 50,
            maxVUs: 200,
        },
    },
    thresholds: {
        http_req_duration: ["p(95)<200"],
        http_req_failed: ["rate<0.01"],
    },
};

export default function () {
    const res = http.get("http://localhost:4000/api/v1/health");
    check(res, {
        "status 200": (r) => r.status === 200,
    });
}
