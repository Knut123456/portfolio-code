import express, { Express, Request, Response } from "express";
import cors from 'cors';

/* import dotenv from "dotenv"; */


const corsOption ={  
    origin: ["10.0.0.8"],
};



const app: Express = express();

app.use(cors(corsOption));
app.get("api", (req, res) => {
    res.json ({fruits: ["apple", "burger"]})
})

const port = 8080
app.listen(8080, () =>{
    console.log("server started on port ", port)
})

