from flask import Flask, request, jsonify, render_template
import sqlite3

app = Flask(__name__)


@app.route("/search", methods=["GET"])
def search():
    con = sqlite3.connect("site.db")
    cur = con.cursor()
    q = request.args.get("q")
    contacts = []

    for row in cur.execute(f"SELECT * FROM phonebook WHERE `names` LIKE '%{q}%'"):
        contacts.append({"name": row[0], "phone": row[1]})

    con.close()
    return jsonify(contacts)

@app.route("/")
def home():
	return render_template("home.html")


if __name__=="__main__":
    app.run(debug=True)
