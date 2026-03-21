import { type RouteConfig, index, layout, route } from '@react-router/dev/routes';

export default [
    layout('./layouts/main.tsx', [
        index('routes/home.tsx'),
        route('signin', 'routes/signin.tsx'),
        route('signup', 'routes/signup.tsx'),
    ]),
    layout('./layouts/dashboard.tsx', [route('projects', 'routes/Projects/projects.tsx')]),
] satisfies RouteConfig;
