const express = require('express');
const app = express();
const port = 3000;

// Serve static files from app/public
app.use(express.static('app/public'));
// Serve node_modules directory
app.use('/node_modules', express.static('node_modules'));

// Define a route for the root URL to serve index.html
app.get('/', (req, res) => {
    res.sendFile(__dirname + '/app/public/index.html');
});

app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});