db.createUser({
    user: "developer",
    pwd: "Q123456q",
    roles: [
        {
            role: "readWrite",
            db: "smart-house",
        },
    ],
});

db.createCollection("rooms");
db.rooms.createIndex(
    { name: 1 },
    { name: "name", unique: true }
);

db.createCollection("powerSwitches")
db.powerSwitches.createIndex(
    { name: 1, room_name: 1 },
    { name: "nameAndRoomName", unique: true }
);

db.createCollection("thermometers")
db.thermometers.createIndex(
    { name: 1, room_name: 1 },
    { name: "nameAndRoomName", unique: true }
);