const auth_service = require("../services/auth.service");

function getAllUsers(req, res) {
  auth_service
    .getAllUsers()
    .then((users) => {
      res.json(users);
    })
    .catch((error) => {
      res.status(500).send({
        message: "Error al obtener los usuarios",
      });
    });
}

function createSessionToken(req, res) {
  const { id } = req.body;
  console.log("hola", id);
  auth_service
    .createSessionToken(id)
    .then((token) => {
      res.json({ token });
    })
    .catch((error) => {
      console.log(error);
      res.status(500).send({
        error: error,
        message: "Error al crear el token",
      });
    });
}

function verifySessionToken(req, res) {
  const { token } = req.body;
  auth_service
    .verifySessionToken(token)
    .then((decoded) => {
      res.json(decoded);
    })
    .catch((error) => {
      res.status(500).send({
        message: "Error al verificar el token",
      });
    });
}

function createUser(req, res) {
  const user = req.body;
  auth_service
    .createUser(user)
    .then((user) => {
      res.json(user);
    })
    .catch((error) => {
      res.status(500).send({
        message: "Error al crear el usuario",
      });
    });
}

module.exports = {
  getAllUsers,
  createSessionToken,
  verifySessionToken,
  createUser,
};
