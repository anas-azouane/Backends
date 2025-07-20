package com.example.springbackend.repository;

import com.example.springbackend.model.Activity;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public class ActivityRepository {

    private final JdbcTemplate jdbc;

    public ActivityRepository(JdbcTemplate jdbc) {
        this.jdbc = jdbc;
    }

    public void insert(Activity activity) {
        String sql = "INSERT INTO activities (id, name) VALUES (?, ?)";
        jdbc.update(sql, activity.getId(), activity.getName());
    }

    public List<Activity> findAll() {
        String sql = "SELECT id, name FROM activities ORDER BY name";
        return jdbc.query(sql, (rs, rowNum) -> new Activity(
            UUID.fromString(rs.getString("id")),
            rs.getString("name")
        ));
    }
}

