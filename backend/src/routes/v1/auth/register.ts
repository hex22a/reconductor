import { API_REGISTER_ENDPOINT_V1 } from '../../../constants';
import { post } from '../../../controllers/auth/register';

export default {
    [API_REGISTER_ENDPOINT_V1]: {
        POST: post,
    }
};
