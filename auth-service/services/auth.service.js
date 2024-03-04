const User_auth = require("../models/authentication.users.model");
const jwt = require("jsonwebtoken");

const getAllUsers = async () => {
  return await User_auth.findAll();
};

const createSessionToken = async (userId) => {
  try {
    const payload = {
      userId: userId,
    };
    const token = jwt.sign(payload, process.env.JWT_SECRET, {
      expiresIn: "16h",
    });
    return token;
  } catch (error) {
    console.log("Error al crear el token", error);
    throw new Error("Error al crear el token");
  }
};

const verifySessionToken = async (token) => {
  return jwt.verify(token, process.env.JWT_SECRET);
};

const createUser = async (user) => {
  console.log(user);
  await User_auth.create({
    id: user.id,
    password: "a",
  });

  const response = await createSessionToken(user.id);

  return {
    token: response,
  };
};

module.exports = {
  getAllUsers,
  createSessionToken,
  verifySessionToken,
  createUser,
};
