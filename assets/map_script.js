function initMap() {
  const mapCanvas = document.getElementById('gmap_canvas');
  if (!mapCanvas) {
    console.error("Map canvas not found!");
    return;
  }

  const locationsData = mapCanvas.dataset.locations;
  if (!locationsData) {
    console.error("Locations data not found!");
    return;
  }

  let locations;
  try {
    locations = JSON.parse(locationsData);
  } catch (e) {
    console.error("Failed to parse locations data:", e);
    return;
  }

  if (locations.length === 0) {
    const defaultCenter = { lat: 51.5074, lng: -0.1278 };
    new google.maps.Map(mapCanvas, {
      zoom: 10,
      center: defaultCenter,
    });
    return;
  }

  const bounds = new google.maps.LatLngBounds();
  locations.forEach(loc => {
    bounds.extend(new google.maps.LatLng(loc.lat, loc.lng));
  });

  const map = new google.maps.Map(mapCanvas, {
    zoom: 12,
  });
  map.fitBounds(bounds);

  locations.forEach(location => {
    new google.maps.Marker({
      position: { lat: location.lat, lng: location.lng },
      map: map,
    });
  });
}
