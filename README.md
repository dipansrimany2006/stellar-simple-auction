🏷️ Simple Auction App

A Simple Auction Application that allows users to create auction listings, place bids, and determine the winning bidder. The project demonstrates basic concepts of web development such as user authentication, CRUD operations, and real-time bidding logic.

📌 Features

🔐 User Authentication

User registration

Login and logout functionality

📦 Create Auction Listings

Users can create new auction items

Add title, description, image, and starting bid

💰 Place Bids

Users can bid on listed items

Bid must be higher than the current highest bid

📊 Auction Status

Shows current highest bid

Displays auction winner when auction closes

⭐ Watchlist (Optional)

Users can save items to their watchlist

🛠️ Technologies Used

Backend: Python / Django

Frontend: HTML, CSS, Bootstrap

Database: SQLite

Version Control: Git & GitHub

📂 Project Structure
simple-auction-app/
│
├── auctions/
│   ├── models.py
│   ├── views.py
│   ├── urls.py
│   ├── forms.py
│
├── templates/
│   ├── index.html
│   ├── listing.html
│   ├── create_listing.html
│
├── static/
│   ├── css/
│
├── db.sqlite3
├── manage.py
└── README.md
⚙️ Installation

1️⃣ Clone the repository

git clone https://github.com/yourusername/simple-auction-app.git

2️⃣ Navigate to project folder

cd simple-auction-app

3️⃣ Install dependencies

pip install -r requirements.txt

4️⃣ Run migrations

python manage.py migrate

5️⃣ Start the server

python manage.py runserver

6️⃣ Open in browser

http://127.0.0.1:8000/
📖 How It Works

Users register and log in to the platform.

Sellers create auction listings with a starting price.

Buyers place bids on items.

The system records the highest bid.

When the auction closes, the highest bidder wins the item.

🚀 Future Improvements

Real-time bidding using WebSockets

Payment gateway integration

Email notifications for bids

Auction timer countdown

Admin dashboard

