import axiosBase from "axios";

export const apiBaseUrl = "http://localhost:8080/api/v1";

export const axios = axiosBase.create({ baseURL: apiBaseUrl });
