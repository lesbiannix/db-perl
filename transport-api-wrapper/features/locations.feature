Feature: Locations
  In order to find train stations
  As a user
  I want to be able to search for locations

  Scenario: Searching for a location
    Given a location query "berlin"
    When I search for locations
    Then I should receive a list of locations
    And the list should contain "Berlin Hbf"