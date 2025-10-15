package com.example.transportapiwrapper;

public class TransportApiWrapper {
    private static native String locations(String query);

    static {
        System.loadLibrary("transport_api_wrapper");
    }

    public static void main(String[] args) {
        String locations = locations("berlin");
        System.out.println(locations);
    }
}