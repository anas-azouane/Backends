package com.example.springbackend.controller;

import com.example.springbackend.model.Activity;
import com.example.springbackend.model.NewActivity;
import com.example.springbackend.repository.ActivityRepository;

import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
public class ActivityController {

    private final ActivityRepository repo;

    public ActivityController(ActivityRepository repo) {
        this.repo = repo;
    }

    @PostMapping("/add")
    public Activity addActivity(@RequestBody NewActivity newActivity) {
        Activity activity = new Activity(UUID.randomUUID(), newActivity.getName());
        repo.insert(activity);
        return activity;
    }

    @GetMapping("/list")
    public List<Activity> listActivities() {
        return repo.findAll();
    }
}

