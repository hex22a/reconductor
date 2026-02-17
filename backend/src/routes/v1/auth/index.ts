import { withErrorHandling } from "@/src/decorators/controller";
import createRegisterRoutes from "./register";

const registerRoutes = createRegisterRoutes(withErrorHandling);

export default { ...registerRoutes };
