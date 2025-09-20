# This is a typical batch processing script
# 
# Run the car assignment procedure
carassignment
# Turn on train printing
setprintem yes
# Run all trains
runalltrains "AllTrains.pdf"
# Run the boxmoves
runboxmoves "BoxMoves.pdf"
# Print yard lists
setprintyards true
# One alphabetical listing of switch lists
setprintalpha yes
setprintatwice no
# One printout of the lists
setprintlist yes
setprintltwice no
# Print dispatcher reports
setprintdispatch yes
# Print the lists
printalllists "AllLists.pdf"
# show cars not moved
showcarsnotmoved
# Show cars moed
showcarmovements yes - -
# show train totals
showtraintotals
# produce an industries report
reportindustries "industries.pdf"
# produce a trains report
reporttrains "trains.pdf"
# produce a cars report
reportcars "cars.pdf"
# produce a cars not moved report
reportcarsnotmoved  "carsnot.pdf"
# produce a car types report 
reportcartypes all ' ' "alltypes.pdf"
# produce a car locations report
reportcarlocations all 0 "CarsAll.pdf"
# produce an analysis report
reportanalysis "Analysis.pdf"
# produce a car ownship report
reportcarowners "TBO" "TimsCars.pdf"
# Normally one would save the cars data for next time
# savecars

