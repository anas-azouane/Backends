package com.example.springbackend.model;

public class NewActivity {
    private String name;

    public NewActivity() {}

    public NewActivity(String name) {
        this.name = name;
    }

    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
}

